fn main() {
    let taco: &str = "Shit";

    if true {
        println!("It is true")
    }

    if false {
        println!("It is false")
    }

    if taco.contains('B') || taco.contains('b') {
        println!("The word {taco} contains a fuckin letter B or b.")
    } else {
      println!("This is some bullshit")
    }
}
