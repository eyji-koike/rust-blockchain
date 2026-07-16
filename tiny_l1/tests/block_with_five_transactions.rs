use tiny_l1::block::*;
use tiny_l1::transaction::*;
use tiny_l1::state::*;

#[test]
fn create_block_with_five_transactions() {
    let state = State::new();
    let parent = Block::genesis(&state, 0);
    let alice = [1u8; 20];
    let bob = [2u8; 20];

    let transactions = vec![
        Transaction::new(alice, bob, 100, 0),
        Transaction::new(alice, bob, 100, 1),
        Transaction::new(alice, bob, 100, 2),
        Transaction::new(alice, bob, 100, 3),
        Transaction::new(alice, bob, 100, 4),
    ];

    let block = Block::new(&parent, transactions, &state, 1);
    assert_eq!(block.header.height, 1);
    assert_eq!(block.header.parent_hash, parent.hash());
    assert!(!block.transactions.is_empty());
}