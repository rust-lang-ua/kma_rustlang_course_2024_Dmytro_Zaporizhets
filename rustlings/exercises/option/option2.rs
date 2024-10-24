// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

// I AM DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // Використання if let для роботи з значеннями типу Some
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // Використання while let для обробки значень в векторі
    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value: {}", integer);
    }
}

