pub mod linked_stack;
pub mod bad;

use linked_stack::*;

fn main() {
    let mut stack = LinkedStack::new();
    dbg!(&stack);
    dbg!(stack.peek());

    stack.push(3);
    dbg!(&stack);
    dbg!(stack.peek());

    stack.push(4);
    dbg!(&stack);

    let elem = stack.pop();
    dbg!(&elem, &stack);

    stack.push(5);
    dbg!(&stack);

    let elem = stack.pop();
    dbg!(&elem, &stack);

    let elem = stack.pop();
    dbg!(&elem, &stack);

    let elem = stack.pop();
    dbg!(&elem, &stack);

    let elem = stack.pop();
    dbg!(&elem, &stack);

    dbg!(stack.peek());

    let mut stack = LinkedStack::new();
    stack.push(8.3);
    dbg!(&stack, stack.peek());

    stack.push(4.2);
    dbg!(&stack, stack.peek());

    for v in stack.iter_mut() {
        if *v <= 4.2 {
            *v += 3.0;
        }
    }
    dbg!(&stack);

    for v in stack {
        println!("v={}", v);
    }

    println!("Hello, world!");
}
