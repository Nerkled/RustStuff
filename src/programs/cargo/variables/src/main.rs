fn main() {
    let mut x = 5;                      // Remember that mut is reffering to the variabel possibly changing in the future, if you do let without mut then it is hardlocked there
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
