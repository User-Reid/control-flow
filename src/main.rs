fn main() {
    let mut seconds: i8 = 21;

    while seconds > 0 {
        if seconds % 2 == 0 {
            println!("{seconds} seconds (even number), skipping 3 seconds...");
            seconds -= 3;
            continue;
        }

        println!("{seconds} seconds until blastoff..");
        seconds -= 1;
    }

    println!("BLASTOFF!🚀");
}
