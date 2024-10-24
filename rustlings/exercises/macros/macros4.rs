// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

// I AM DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // Додаємо крапку з комою, щоб розділити два блоки
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
