use std::io::{self, Write};

fn main(){

    loop {
        print!("What is your grade in english ? ");
        io::stdout()
            .flush()
            .expect("failed to flush");
    
        let mut grade = String::new();
    
        io::stdin()
            .read_line(&mut grade)
            .expect("failed to read line");
    
        let grade :i32 = match grade.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("is not number :(");
                    continue;
                }
            };
    
        if grade >= 95 && grade <= 100 {
            println!("your grade is A+");
            break;
        } else if grade >= 90 && grade < 95 {
            println!("your grade is A");
        } else if grade >= 80 && grade < 90{
            println!("your grade is B+");
        } else if grade >= 70 && grade < 80{
            println!("your grade is B");
        } else if grade >= 60 && grade < 70{
            println!("your grade is C");
        } else if grade >= 50 && grade < 60{
            println!("your grade is D");
        } else if grade >= 0 && grade < 50 {
            println!("your grade is E");
        } else {
            println!("invalid grade");
        }
    
    }

}