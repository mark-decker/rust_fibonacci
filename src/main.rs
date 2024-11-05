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
 
        let n: u32 = match n_th
            .trim()
            .parse() {   //Handle the error
                Ok(num) => num,
                Err(_) => continue,
            };
        break n
    };

    //could add all values to a vec to print on a single line
    //print on multiple lines for simplicity
    for i in 0..n_th {
        println!("Fibonacci Sequence of {} is {}", i, gen_fib(i));
    }

}

//define a recursive fibonacci funtion that returns next number
fn gen_fib (n_th: u32) -> u32 {
    if n_th <= 0 {
        return 0;
    } else if n_th == 1 {
        return 1;
    } else {
        return gen_fib(n_th-1)  + gen_fib(n_th-2);
    }
}


