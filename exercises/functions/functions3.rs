// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE
//takeaway: type stability is important.
fn main() {
    let num:i32 = 10;
    call_me(num);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
