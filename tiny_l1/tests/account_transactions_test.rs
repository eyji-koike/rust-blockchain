use tiny_l1::*;

/// Helper to build a 20-byte address from a single byte repeated.
fn addr(byte: u8) -> [u8; 20] {
    [byte; 20]
}

#[test]
fn create_account_send_transaction_and_verify_balance() {
    let mut state = State::new();

    let alice = addr(0x01);
    state.accounts.insert(
        alice,
        Account {
            address: alice,
            balance: 1_000,
            nonce: 0,
        },
    );

    let bob = addr(0x02);
    state.accounts.insert(
        bob,
        Account {
            address: bob,
            balance: 0,
            nonce: 0,
        },
    );

    let tx = Transaction {
        from: alice,
        to: bob,
        value: 250,
        nonce: 0,
        signature: None,
    };

    state.apply(&tx).expect("transaction should apply cleanly");

    assert_eq!(state.accounts.get(&alice).unwrap().balance, 750);
    assert_eq!(state.accounts.get(&alice).unwrap().nonce, 1);
    assert_eq!(state.accounts.get(&bob).unwrap().balance, 250);
    assert_eq!(state.accounts.get(&bob).unwrap().nonce, 0);
}

#[test]
fn applying_transaction_creates_receiver_if_missing() {
    let mut state = State::new();

    let alice = addr(0x01);
    let bob = addr(0x02);

    state.accounts.insert(
        alice,
        Account {
            address: alice,
            balance: 500,
            nonce: 0,
        },
    );

    let tx = Transaction {
        from: alice,
        to: bob,
        value: 100,
        nonce: 0,
        signature: None,
    };

    state.apply(&tx).unwrap();

    assert_eq!(state.accounts.get(&alice).unwrap().balance, 400);
    assert_eq!(state.accounts.get(&bob).unwrap().balance, 100);
}

#[test]
fn insufficient_balance_is_rejected() {
    let mut state = State::new();

    let alice = addr(0x01);
    let bob = addr(0x02);

    state.accounts.insert(
        alice,
        Account {
            address: alice,
            balance: 50,
            nonce: 0,
        },
    );

    let tx = Transaction {
        from: alice,
        to: bob,
        value: 100,
        nonce: 0,
        signature: None,
    };

    assert!(state.apply(&tx).is_err());
}

#[test]
fn invalid_nonce_is_rejected() {
    let mut state = State::new();

    let alice = addr(0x01);
    let bob = addr(0x02);

    state.accounts.insert(
        alice,
        Account {
            address: alice,
            balance: 1_000,
            nonce: 5,
        },
    );

    let tx = Transaction {
        from: alice,
        to: bob,
        value: 10,
        nonce: 0,
        signature: None,
    };

    assert!(state.apply(&tx).is_err());
}