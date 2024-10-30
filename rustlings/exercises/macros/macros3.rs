// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// I AM DONE

#[macro_use] // Bring macros from the `macros` module into scope
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // Now the macro is available in this scope
}
