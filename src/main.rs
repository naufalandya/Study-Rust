// src/main.rs
mod geometry;
use std::io::{self, Write};

fn main() {
    loop {
        println!("What do you want to do?");
        println!("1. 2D Calculations");
        println!("2. 3D Calculations");
        println!("3. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); 

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => handle_2d_calculations(),
            "2" => handle_3d_calculations(),
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("You didn't choose any valid option! Try again."),
        }
    }
}

fn handle_2d_calculations() {
    println!("Choose a 2D figure:");
    println!("1. Rectangle");
    println!("2. Circle");
    println!("3. Triangle");

    let mut figure_choice = String::new();
    io::stdin().read_line(&mut figure_choice).unwrap();
    let figure_choice = figure_choice.trim();

    match figure_choice {
        "1" => {
            let (length, width) = get_two_inputs("length", "width");
            let area = geometry::geometry::rectangle_area(length, width);
            println!("Area of rectangle: {:.2}", area);
        }
        "2" => {
            let radius = get_input("radius");
            let area = geometry::geometry::circle_area(radius);
            println!("Area of circle: {:.2}", area);
        }
        "3" => {
            let (base, height) = get_two_inputs("base", "height");
            let area = geometry::geometry::triangle_area(base, height);
            println!("Area of triangle: {:.2}", area);
        }
        _ => println!("Invalid choice!"),
    }
}

fn handle_3d_calculations() {
    println!("Choose a 3D figure:");
    println!("1. Cube");
    println!("2. Sphere");
    println!("3. Cylinder");

    let mut figure_choice = String::new();
    io::stdin().read_line(&mut figure_choice).unwrap();
    let figure_choice = figure_choice.trim();

    match figure_choice {
        "1" => {
            let side = get_input("side");
            let volume = geometry::geometry::cube_volume(side);
            println!("Volume of cube: {:.2}", volume);
        }
        "2" => {
            let radius = get_input("radius");
            let volume = geometry::geometry::sphere_volume(radius);
            println!("Volume of sphere: {:.2}", volume);
        }
        "3" => {
            let (radius, height) = get_two_inputs("radius", "height");
            let volume = geometry::geometry::cylinder_volume(radius, height);
            println!("Volume of cylinder: {:.2}", volume);
        }
        _ => println!("Invalid choice!"),
    }
}

fn get_input(prompt: &str) -> f64 {
    print!("Enter {}: ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().expect("Please enter a valid number")
}

fn get_two_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
    let input1 = get_input(prompt1);
    let input2 = get_input(prompt2);
    (input1, input2)
}
