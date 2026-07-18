use std::thread;
use std::time::Duration;

use tiny_l1::block::Block;
use tiny_l1::consensus::ConsensusEngine;
use tiny_l1::state::State;

fn main() {
    let state = State::new();
    let mut chain: Vec<Block> = vec![Block::genesis(&state, 0)];
    let mut consensus = ConsensusEngine::new(vec![[1u8; 20]], 1);

    loop {
        if let Some(block) = consensus.step() {
            chain.push(block);
            println!(
                "Block {} finalized. Chain height: {}",
                chain.last().unwrap().header.height,
                chain.len()
            );
        }

        thread::sleep(Duration::from_millis(200));
    }
}