fn main() {
    let mut seconds: i8 = 20;

    loop {
        if seconds == 0 {
            println!("BLASTOFF!🚀");
            break;
        }

        println!("{seconds} seconds until blastoff..");
        seconds -= 1;
    }
}
