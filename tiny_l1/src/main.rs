use std::time::{SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand};

use tiny_l1::account::Account;
use tiny_l1::block::Block;
use tiny_l1::consensus::{ConsensusEngine, Vote, VoteType};
use tiny_l1::state::State;
use tiny_l1::transaction::Transaction;

#[derive(Parser)]
#[command(name = "tiny_l1")]
#[command(about = "A tiny layer-1 blockchain simulator")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a deterministic consensus simulation and print committed blocks.
    Simulate {
        #[arg(short, long, default_value = "5")]
        blocks: usize,
        #[arg(short, long, default_value = "4")]
        validators: usize,
        #[arg(short, long, default_value = "3")]
        threshold: usize,
    },
    /// Run a node that produces one block and exits.
    Node,
    /// Generate a dummy 20-byte account address.
    Account,
    /// Print a genesis block.
    Genesis,
    /// Seed state with accounts and run transfers through consensus.
    Transfer {
        #[arg(short, long, default_value = "3")]
        blocks: usize,
        #[arg(short, long, default_value = "1000")]
        initial_balance: u64,
        #[arg(short, long, default_value = "10")]
        transfers_per_block: usize,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Simulate {
            blocks,
            validators,
            threshold,
        } => simulate(blocks, validators, threshold),
        Commands::Node => run_node(),
        Commands::Account => println!("0x{}", hex::encode([1u8; 20])),
        Commands::Genesis => {
            let state = State::new();
            let block = Block::genesis(&state, current_timestamp());
            println!("{:#?}", block);
        }
        Commands::Transfer {
            blocks,
            initial_balance,
            transfers_per_block,
        } => run_transfer_simulation(blocks, initial_balance, transfers_per_block),
    }
}

fn simulate(blocks: usize, validators: usize, threshold: usize) {
    if validators == 0 || threshold == 0 || threshold > validators {
        eprintln!("invalid consensus config: need 0 < threshold <= validators");
        std::process::exit(1);
    }

    let validators: Vec<[u8; 20]> = (0..validators)
        .map(|i| [i as u8; 20])
        .collect();

    let state = State::new();
    let genesis = Block::genesis(&state, current_timestamp());
    let mut chain = vec![genesis.clone()];

    let mut engine = ConsensusEngine::new(validators.clone(), threshold);

    for height in 1..=blocks {
        let parent = chain.last().unwrap();
        log::info!("--- height {} ---", height);
        let proposed = engine.propose(parent, vec![], &state, current_timestamp());
        log::info!(
            "proposed block {} hash=0x{}",
            height,
            hex::encode(proposed.hash())
        );

        for validator in validators.iter().take(threshold) {
            engine.add_prevote(Vote {
                validator: *validator,
                height: engine.height,
                round: engine.round,
                block_hash: proposed.hash(),
                vote_type: VoteType::Prevote,
            });
            engine.add_precommit(Vote {
                validator: *validator,
                height: engine.height,
                round: engine.round,
                block_hash: proposed.hash(),
                vote_type: VoteType::Precommit,
            });
        }

        let committed = engine.run_to_commit().expect("block should commit");
        chain.push(committed.clone());
        println!(
            "Committed block {} hash=0x{} height={} tx_count={}",
            height,
            hex::encode(committed.hash()),
            committed.header.height,
            committed.transactions.len()
        );
    }
}

fn run_node() {
    let state = State::new();
    let genesis = Block::genesis(&state, current_timestamp());
    let mut chain = vec![genesis.clone()];
    let mut engine = ConsensusEngine::new(vec![[1u8; 20]], 1);

    let parent = chain.last().unwrap();
    let proposed = engine.propose(parent, vec![], &state, current_timestamp());

    engine.add_prevote(Vote {
        validator: [1u8; 20],
        height: engine.height,
        round: engine.round,
        block_hash: proposed.hash(),
        vote_type: VoteType::Prevote,
    });
    engine.add_precommit(Vote {
        validator: [1u8; 20],
        height: engine.height,
        round: engine.round,
        block_hash: proposed.hash(),
        vote_type: VoteType::Precommit,
    });

    if let Some(block) = engine.run_to_commit() {
        chain.push(block.clone());
        println!(
            "Block {} finalized. Chain height: {}",
            block.header.height,
            chain.len()
        );
    } else {
        println!("No block committed");
    }
}

fn run_transfer_simulation(blocks: usize, initial_balance: u64, transfers_per_block: usize) {
    let alice = [1u8; 20];
    let bob = [2u8; 20];

    let mut state = State::new();
    state.accounts.insert(
        alice,
        Account {
            address: alice,
            balance: initial_balance,
            nonce: 0,
        },
    );

    let genesis = Block::genesis(&state, current_timestamp());
    let mut chain = vec![genesis.clone()];
    let mut engine = ConsensusEngine::new(vec![alice], 1);
    let mut nonce = 0u64;

    for height in 1..=blocks {
        let parent = chain.last().unwrap();

        let mut transactions = Vec::new();
        for _ in 0..transfers_per_block {
            let tx = Transaction::new(alice, bob, 10, nonce);
            nonce += 1;
            if let Err(e) = state.apply(&tx) {
                log::warn!("transaction failed: {}", e);
                continue;
            }
            transactions.push(tx);
        }

        log::info!("--- height {} with {} transfers ---", height, transactions.len());
        let proposed = engine.propose(parent, transactions, &state, current_timestamp());
        log::info!(
            "proposed block {} hash=0x{} state_root=0x{}",
            height,
            hex::encode(proposed.hash()),
            hex::encode(proposed.header.state_root)
        );

        engine.add_prevote(Vote {
            validator: alice,
            height: engine.height,
            round: engine.round,
            block_hash: proposed.hash(),
            vote_type: VoteType::Prevote,
        });
        engine.add_precommit(Vote {
            validator: alice,
            height: engine.height,
            round: engine.round,
            block_hash: proposed.hash(),
            vote_type: VoteType::Precommit,
        });

        let committed = engine.run_to_commit().expect("block should commit");
        chain.push(committed.clone());

        let alice_balance = state.accounts.get(&alice).map(|a| a.balance).unwrap_or(0);
        let bob_balance = state.accounts.get(&bob).map(|a| a.balance).unwrap_or(0);

        println!(
            "Committed block {} hash=0x{} height={} tx_count={} alice={} bob={}",
            height,
            hex::encode(committed.hash()),
            committed.header.height,
            committed.transactions.len(),
            alice_balance,
            bob_balance
        );
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs()
}