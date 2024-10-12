use std::io::{self, Write};

fn main(){
    let mut name = String::new();

    print!("Please enter your name : ");
    io::stdout()
        .flush()
        .expect("failed to flush");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed name");

    println!("Your name is {name}");

    //kalo const tipe data nya harus dijelasin
    const X: i32 = 5;
    const Y: i32 = 10;
    const Z: i32 = X + Y;

    println!("hasil x + y adalah {Z}");

    // Tipe data kalau bisa langsung di deklrasikan

    let angka_satu: i32 = 1500;
    let angka_dua: i32 = 1090;
    let total = angka_dua + angka_dua + angka_satu;
    println!("{total}");

    let angka_satu: u32 = "42".parse().expect("Not a number!");
    println!("{angka_satu}");

    //skalar

    // A scalar type represents a single value. 
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. 
    // You may recognize these from other programming languages. Let’s jump into how they work in Rust.


    //number

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}");

    // Boolean

    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("{t}, {f}");

    // Char
    let c = "z"; //bukan char
    let d = 'A';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{} {} {} {}", c, d, z, heart_eyed_cat);
    println!("{}", c);


    // Compound Types
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!(" {x} {y} {z} ");

    let satu = tup.0;
    let dua = tup.1;
    let tiga = tup.2;

    println!(" {satu} {dua} {tiga} ");

    //Array

    let days = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
    println!("{:?}", days); // Print the array of days

    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums); // Print the array of numbers

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("{first}, {second}");

    let b = [1, 2, 3, 4, 5, 6];

    let mut input = String::new();

    print!("Input the index of the array element : ");
        io::stdout()
            .flush()
            .expect("failed to flush lol");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

    let input :usize = input
        .trim()
        .parse()
        .expect("must be a number !");

    println!("the value of an array on index {input} is {} ", b[input]);

    hello_world();
}

//satu

fn hello_world(){
    println!("Hello World !");

    let mut surface= String::new();

    print!("Please enter the surface number ! : ");
        io::stdout()
            .flush()
            .expect("failed to flush");

        io::stdin()
            .read_line(&mut surface)
            .expect("failed to read line");

    let surface :i32 = surface
            .trim()
            .parse()
            .expect("surface is not a number");

    let mut height = String::new();
    
    print!("please enter the height number ! : ");
        io::stdout()
            .flush()
            .expect("failed to flush");

        io::stdin()
            .read_line(&mut height)
            .expect("failed to read line");

    let height:i32 = height
        .trim()
        .parse()
        .expect("height is not a number");

    let result :f32 = calculate_triangle_area(surface, height);
    println!("the area of triangle with surface is {surface} and {height} is {result}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn calculate_triangle_area(surface: i32, height: i32) -> f32 {
    let result: f32 = 0.5 * (surface as f32) * (height as f32);
    result
}