#![feature(specialization)]

pub mod bad;

pub mod linked_stack;
pub mod shared_linked_list;
pub mod linked_deque;
pub mod linked_queue;

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

    let mut stack = LinkedStack::new();
    stack.push(3);
    stack.push(4);
    stack.push(5);
    dbg!(&stack);

    let mut new_stack = None;
    let mut iter = stack.iter_mut_book();
    while let Some(v) = iter.next() {
        dbg!(v, iter.peek());
        if let Some(4) = iter.peek() {
            new_stack = Some(iter.split_after());
        }
    }
    dbg!(&stack, &new_stack);

    let mut iter = stack.iter();
    println!("stack.iter.peek={:?}", iter.peek());
    //stack.push(44); // 不可以在持有 iter 的情况下更改
    //println!("stack.iter.peek={:?}", iter.peek());

    let mut into_iter = stack.into_iter();
    //println!("into_iter.0={:?}", into_iter.0); // tuple struct 默认还是 private 的, 可加 pub

    let mut stack = LinkedStack::new();
    stack.push(3);
    stack.push(4);
    stack.push(5);
    dbg!(&stack);

    let mut iter = stack.iter_mut_book();
    while let Some(v) = iter.next() {
        dbg!(v, iter.peek());
        if let Some(4) = iter.peek() {
            iter.insert_after(88);
        }
    }
    dbg!(&stack);

    println!("Hello, world!");
}
