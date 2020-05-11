fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);

    // Shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    let y = y * 2;
    println!("The new value of y is: {}", y);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of the original \"spaces\" string is: {}", spaces);

    // Floating-point numbers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Accessing elements from tuples (different elements)
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // With explicit type
    let a = [3; 5]; // Same element repeated ("five threes")

    // Accessing elements from arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}
