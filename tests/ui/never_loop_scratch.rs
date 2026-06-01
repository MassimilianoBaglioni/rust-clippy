#![feature(try_blocks)]
#![expect(clippy::eq_op, clippy::single_match, clippy::while_immutable_condition)]
//@no-rustfix

use std::arch::asm;

fn main() {
    'bar: for _ in 0..100 {
        //~^ never_loop
        loop {
            //~^ never_loop
            println!("This will still run");
            break 'bar;
        }
    }

    'foo: for _ in 0..100 {
        //~^ never_loop
        loop {
            //~^ never_loop
            println!("This will still run");
            loop {
                //~^ never_loop
                println!("This will still run");
                break 'foo;
            }
        }
    }
}
