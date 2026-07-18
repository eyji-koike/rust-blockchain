use crate::prelude::*;
use crate::block::Block;
use crate::state::State;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusState {
    Idle,
    Proposing,
    PreVote,
    PreCommit,
    Commit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoteType {
    Prevote,
    Precommit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vote {
    pub validator: [u8; 20],
    pub height: u64,
    pub round: u64,
    pub block_hash: Hash,
    pub vote_type: VoteType,
}

pub struct ConsensusEngine {
    pub state: ConsensusState,
    pub height: u64,
    pub round: u64,
    pub proposed_block: Option<Block>,
    pub pre_votes: HashMap<[u8; 20], Vote>,
    pub pre_commits: HashMap<[u8; 20], Vote>,
    pub validators: Vec<[u8; 20]>,
    pub threshold: usize,
}

impl ConsensusEngine {
    pub fn new(validators: Vec<[u8; 20]>, threshold: usize) -> Self {
        Self {
            state: ConsensusState::Idle,
            height: 1,
            round: 0,
            proposed_block: None,
            pre_votes: HashMap::new(),
            pre_commits: HashMap::new(),
            validators,
            threshold,
        }
    }

    pub fn propose(&mut self, parent: &Block, transactions: Vec<crate::transaction::Transaction>, state: &State, timestamp: u64) -> Block {
        let block = Block::new(parent, transactions, state, timestamp);
        self.proposed_block = Some(block.clone());
        self.state = ConsensusState::Proposing;
        block
    }

    pub fn start_round(&mut self, proposed: Block) {
        self.proposed_block = Some(proposed);
        self.state = ConsensusState::Proposing;
        self.pre_votes.clear();
        self.pre_commits.clear();
    }

    pub fn add_prevote(&mut self, vote: Vote) {
        if self.validators.contains(&vote.validator) {
            self.pre_votes.insert(vote.validator, vote);
        }
    }

    pub fn add_precommit(&mut self, vote: Vote) {
        if self.validators.contains(&vote.validator) {
            self.pre_commits.insert(vote.validator, vote);
        }
    }

    pub fn step(&mut self) -> Option<Block> {
        match self.state {
            ConsensusState::Idle => None,
            ConsensusState::Proposing => {
                log::info!("height={} round={} -> PreVote", self.height, self.round);
                self.state = ConsensusState::PreVote;
                None
            }
            ConsensusState::PreVote => {
                let vote_count = self.pre_votes.len();
                log::info!(
                    "height={} round={} PreVote: {}/{} votes",
                    self.height,
                    self.round,
                    vote_count,
                    self.threshold
                );
                if vote_count >= self.threshold {
                    log::info!("height={} round={} -> PreCommit", self.height, self.round);
                    self.state = ConsensusState::PreCommit;
                }
                None
            }
            ConsensusState::PreCommit => {
                let commit_count = self.pre_commits.len();
                log::info!(
                    "height={} round={} PreCommit: {}/{} votes",
                    self.height,
                    self.round,
                    commit_count,
                    self.threshold
                );
                if commit_count >= self.threshold {
                    log::info!("height={} round={} -> Commit", self.height, self.round);
                    self.state = ConsensusState::Commit;
                    return self.proposed_block.clone();
                }
                None
            }
            ConsensusState::Commit => {
                log::info!("height={} round={} block committed; resetting", self.height, self.round);
                self.height += 1;
                self.round = 0;
                self.pre_votes.clear();
                self.pre_commits.clear();
                self.proposed_block = None;
                self.state = ConsensusState::Idle;
                None
            }
        }
    }

    pub fn run_to_commit(&mut self) -> Option<Block> {
        for _ in 0..10 {
            if let Some(block) = self.step() {
                return Some(block);
            }
        }
        None
    }
}