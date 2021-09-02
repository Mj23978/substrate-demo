fn main() {
    let x = 5;
    println!("x is : {}", x);
    // x = 6; cant do that cuase x is immutable
    let mut y = 5;
    y = x + 1;
    println!("y is : {}", y);
}
