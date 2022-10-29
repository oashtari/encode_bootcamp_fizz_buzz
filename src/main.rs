fn main() {
    println!("Welcome to the Encode bootcamp!");

    fizz_buzz(301);
}

fn fizz_buzz(number: i16) {
    let mut count = 1;
    let mut fb_count = 0;
    // let mut fizz_count = 0;
    // let mut buzz_count = 0;
    while count <= number {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizz buzz");
            fb_count += 1;
        } else if count % 3 == 0 {
            println!("fizz");
            // fizz_count += 1;
        } else if count % 5 == 0 {
            println!("buzz");
            // buzz_count += 1;
        }
        count += 1;
    }
    println!("fizz buzz count: {} ", fb_count);
    // println!("fizz  count: {} ", fizz_count);
    // println!("buzz count: {} ", buzz_count);
}
