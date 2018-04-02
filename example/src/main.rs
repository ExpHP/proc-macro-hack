#[macro_use]
extern crate demo_hack;

id!{
    fn two() -> i32 {
        let x = 2; // trigger a warning so we can see the span...
        2
    }
}

fn main() {
    let nine = add_one!(two()) + add_one!(2 + 3);
    println!("nine = {}", nine);
}
