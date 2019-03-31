fn main() {
    const FIZZ: &str = "Fizz";
    const BUZZ: &str = "Buzz";
    const FIZZ_BUZZ: &str = "Fizz Buzz";

    for x in 1..101 {
        if x % 15 == 0 { 
            println!("{}:{}", x, FIZZ_BUZZ);
        } else if x % 3 == 0 {
            println!("{}:{}", x, FIZZ);
        } else if x % 5 == 0 {
            println!("{}:{}", x, BUZZ);
        } else {
            println!("{}", x);
        }
    }
}