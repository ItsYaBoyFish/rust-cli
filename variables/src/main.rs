fn main() {
    let mut x = 5;
    println!("The Value of X is {x}");
    x = 6;
    println!("The Value of X is {x}");

    // This can never be mutable. 
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
