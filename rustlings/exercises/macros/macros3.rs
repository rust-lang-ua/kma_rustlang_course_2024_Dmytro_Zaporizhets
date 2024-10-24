// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// I AM DONE

use crate::my_macro;

mod macros {
    #[macro_export] // Робимо макрос доступним для інших модулів
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // Тепер макрос доступний у глобальному просторі
}
