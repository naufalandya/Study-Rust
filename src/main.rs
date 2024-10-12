// use std::io;
use std::io::{self, Write};
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let name: &str = "Naufal Andya Faiz";

    println!("Hello world {}", name);

    print!("What is your age ? ");
    io::stdout()
        .flush()
        .expect("Failed to flush");

    let mut age: String = String::new();

    // print!("What is your age"); //jangan disni

    io::stdin()
        .read_line(&mut age)
        .expect("failed to read line");

    println!("Your name is {} with age {}", name, age);
 
    let x = 5;
    let age: i32 = age.trim().parse().expect("please enter number!");

    let y: i32;
    y = x + age;
    println!("If the value of x is {x}, and your age is {age}, then total of y (lucky number) = x + age + 10 (random anomaly number lol) is {}", y + 10);


    // Shadowing
    let age: String = "20".to_string();
    println!("String age: {}", age); // age is a String

    let age: i32 = age.trim().parse().unwrap();
    println!("Integer age: {}", age); // age is now an i32 after shadowing

    println!("But hey now lets play guessing games !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is {secret_number}");

    loop { //allowing multiple input

    print!("please input your number : ");
    io::stdout()
        .flush()
        .expect("Failed to flush");


    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("failed to read line");

    println!("you guessed {number}");

    //shadowing again

        let number: u32 = number.trim().parse().expect("Please type a number!");

        //Comparing
    
        match number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");     
                break; //stop loop
    
            }
        }
    }

}