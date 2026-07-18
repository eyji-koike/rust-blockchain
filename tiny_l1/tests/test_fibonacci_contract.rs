use tiny_l1::vm::*;
use tiny_l1::state::*;

#[test]
fn test_fibonacci_contract() {
    
    // Test implementation here
    let code = vec![
        Opcode::Push(0), Opcode::Store(0),
        Opcode::Push(1), Opcode::Store(1),
        Opcode::Push(10), Opcode::Store(2), // Store the number of Fibonacci numbers to calculate

        //loop: index 7
        Opcode::Load(0),
        Opcode::Load(1),
        Opcode::Add,
        Opcode::Store(0), // a = a + b
        Opcode::Load(1),
        Opcode::Store(3), // temp = old b
        Opcode::Load(0),
        Opcode::Store(1), // b = new a
        Opcode::Load(3),
        Opcode::Store(0), // a = temp (old b)
        Opcode::Load(2),
        Opcode::Push(1),
        Opcode::Sub,
        Opcode::Dup(0),
        Opcode::Store(2),
        Opcode::JumpIfZero(23),
        Opcode::Jump(6), // Jump to beginning of loop
        Opcode::Halt,
    ];
    let mut vm = VM::new(code, State::new(), 1000);
    vm.run().expect("VM should run without errors");

    assert_eq!(vm.memory.get(&0), Some(&55)); // 10th Fibonacci number is 55
    assert_eq!(vm.memory.get(&1), Some(&89)); // 9th Fibonacci number is 89
}