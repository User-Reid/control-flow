fn main() {
    let number: i8 = 8;

    match number {
        2 | 4 | 6 | 8 => {
            println!("{number} is even")
        }

        1 | 3 | 5 | 7 => {
            println!("{number} is odd")
        }
        _ => {
            println!("unknown for now")
        }
    }
}
