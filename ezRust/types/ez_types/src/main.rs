fn main() {
    let slice = "Hello";
    println!("Slice is {} bytes", slice.len());
    let slice2 = "😎";
    println!("Slice2 is {} bytes", slice2.len());

    // Emojis and other special characters take up 4 bytes

    let slice3 = "Hello!!";
    println!("Slice3 is {} bytes but only {} characters.", slice3.len(), slice3.chars().count()); 
    let slice4 = "😎!!";
    println!("Slice4 is {} bytes but only {} characters.", slice4.len(), slice4.chars().count());
}
