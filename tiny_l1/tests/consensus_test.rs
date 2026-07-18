use tiny_l1::consensus::*;
use tiny_l1::block::Block;
use tiny_l1::state::State;

fn addr(byte: u8) -> [u8; 20] {
    [byte; 20]
}

#[test]
fn single_validator_reaches_commit() {
    let state = State::new();
    let validator = addr(1);

    let mut engine = ConsensusEngine::new(vec![validator], 1);
    let genesis = Block::genesis(&state, 0);
    let proposed = engine.propose(&genesis, vec![], &state, 1);

    engine.add_prevote(Vote {
        validator,
        height: 1,
        round: 0,
        block_hash: proposed.hash(),
        vote_type: VoteType::Prevote,
    });

    engine.add_precommit(Vote {
        validator,
        height: 1,
        round: 0,
        block_hash: proposed.hash(),
        vote_type: VoteType::Precommit,
    });

    let committed = engine.run_to_commit();

    assert!(committed.is_some());
    assert_eq!(committed.unwrap().hash(), proposed.hash());
    assert_eq!(engine.state, ConsensusState::Commit);
}

#[test]
fn four_validators_commit_same_block() {
    let validators: Vec<[u8; 20]> = (1..=4).map(addr).collect();
    let threshold = 3;
    let state = State::new();

    let genesis = Block::genesis(&state, 0);
    let proposed = Block::new(&genesis, vec![], &state, 1);

    let mut engines: Vec<_> = validators
        .iter()
        .map(|_| ConsensusEngine::new(validators.clone(), threshold))
        .collect();

    for engine in engines.iter_mut() {
        engine.start_round(proposed.clone());
    }

    for (i, engine) in engines.iter_mut().enumerate() {
        let validator = validators[i];
        engine.add_prevote(Vote {
            validator,
            height: 1,
            round: 0,
            block_hash: proposed.hash(),
            vote_type: VoteType::Prevote,
        });
        engine.add_precommit(Vote {
            validator,
            height: 1,
            round: 0,
            block_hash: proposed.hash(),
            vote_type: VoteType::Precommit,
        });
    }

    for engine in engines.iter_mut() {
        for validator in validators.iter().take(threshold) {
            engine.add_prevote(Vote {
                validator: *validator,
                height: 1,
                round: 0,
                block_hash: proposed.hash(),
                vote_type: VoteType::Prevote,
            });
            engine.add_precommit(Vote {
                validator: *validator,
                height: 1,
                round: 0,
                block_hash: proposed.hash(),
                vote_type: VoteType::Precommit,
            });
        }
    }
}

#[test]
fn consensus_does_not_commit_without_quorum() {
    let state = State::new();
    let validators: Vec<[u8; 20]> = (1..=4).map(addr).collect();
    let threshold = 3;

    let mut engine = ConsensusEngine::new(validators, threshold);
    let genesis = Block::genesis(&state, 0);
    let proposed = engine.propose(&genesis, vec![], &state, 1);

    engine.add_prevote(Vote {
        validator: addr(1),
        height: 1,
        round: 0,
        block_hash: proposed.hash(),
        vote_type: VoteType::Prevote,
    });

    let committed = engine.run_to_commit();

    assert!(committed.is_none());
    assert_eq!(engine.state, ConsensusState::PreVote);
}