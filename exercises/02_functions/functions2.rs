// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(10);
}

// Fix missing type call_me(num) -> call_me(num: u32)
// NOTE: Added u32 to avoid receiving negative numbers.
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
