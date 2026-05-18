// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

/*
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
    swap the position of the macro definition and the main function, and make it compile.
*/
