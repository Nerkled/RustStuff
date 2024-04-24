fn main() {
    let mut x = 5;                      // Remember that mut is reffering to the variabel possibly changing in the future, if you do let without mut then it is hardlocked there
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Const code, const is always immutable and u can't change it
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // The above const name is standard naming convention in Rust
    // Here is some helpful const stuff : https://doc.rust-lang.org/reference/const_eval.html 
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    let y = 5; 

    let y= y +1; // shadowed once, using let again here essetialy creates a whole new variable 
    {
        let y = y * 2;  // shadowed twice 
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}
