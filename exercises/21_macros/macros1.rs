// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    ($val:expr) => {
        println!("Check out my macro!{}",$val);
    };
}

fn main() {
    my_macro!(34);
}
