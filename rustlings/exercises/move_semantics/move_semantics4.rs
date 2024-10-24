// move_semantics4.rs
// Refactor this code so that instead of having `vec0` and creating the vector
// in `fn main`, we create it within `fn fill_vec` and transfer the
// freshly created vector from fill_vec to its caller.
// Execute `rustlings hint move_semantics4` for hints!

// I AM DONE

fn main() {
    let mut vec1 = fill_vec(); // Створюємо vec1 без передачі vec0

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` більше не приймає аргументи
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // Створюємо новий вектор

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

