fn main() {
    let season: &str = "fall";

    if season == "summer" {
        println!("summer")
    } else if season == "winter" {
        println!("winter")
    } else {
        println!("its raining")
    }

    match season {
        "summer" => println!("summer"),
        "winter" => println!("winter"),
        _ => println!("lots of rain"),
    }
}
