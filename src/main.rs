//Mark Decker 11/5/2024
//Rust-lang chapter 3 practice
//generate the Nth Fibonacci number
//
//Must prompt user for N
//print the Fibonacci sequence for N

use std::io;

fn main() {

    let n_th: u32 = loop {

        let mut n_th = String::new();

        println!("Please input a positive integer:");

        io::stdin()
            .read_line(&mut n_th)
            .expect("Failed to read line");

        println!("You have enetered: {n_th}");
 
        let n_th: u32 = match n_th
            .trim()
            .parse() {   //Handle the error
                Ok(num) => num,
                Err(_) => continue,
            };
        break n_th
    };


    println!("Hello, world!");
}
