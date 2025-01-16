fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is also a mutable reference to x

    *y = 10; // Modify x through y
    *z = 15; // Modify x through z

    println!("x = {}", x); // This might lead to data races and unpredictable output
}