//! # PRIM-SYS-R
//! 
//! Prim-Sys-r is the rust port of the now deprecated Prim-Sys, originaly written in C#,
//! Prim-Sys-r is Prim-Sys rewriten and expanded in the Rust programming lenguaje taken advantage of its low-level features
//!
//! ## Features
//!     -Taking user imput  
//!     -Realizing the main execution loop
//!     -Calling handlers of other modules and crates

use std::io;
use crate::math::math_handler;
mod math;
mod commons;

/// ## Main Fuction
/// -Command entry point
/// -Executes the main loop including, getting user input , seperating it into command and argument, and passing to the respective functions.
/// -Handling Errors in user input
fn main() {
    println!("
    Welcome to Prim-Sys Rust \n
    Consult provided documentation in the docs folder provided with the installed release. \n 
    Version 0.0.1 [DEV] \n
    Brought to you by the Prim-Sys dev team and Collaborators. 
    ");
    'primery:loop {
        print!(">");
        let command = {
            let mut com= String::new(); 
            io::stdin().read_line(&mut com).expect("Failed to read line"); // Assings a value to the empy string com
            com //assings com as a non mut
        }; 
        
        let mut command = command.trim().split_whitespace().collect::<Vec<&str>>(); 
        //Splits command into command and argument via a a str reference
        if command.len() == 0 {
            print!("No command entered");
            continue;
        }
        if command.len() == 1 {
            command.push("");
        }
        match command[0] {
            "exit" => break 'primery,
            "help" => help(command[1]),
            "math" => math(command[1]),
            _ => println!("Unknow command")
        } //Matches the command and passes the argument into the pertinent function
    }
}

/// ## Help Function
/// Only function hardcoded into the main module
/// provides description and argument list for the default functions
/// -Math
/// -Help
/// -Exit
/// -lsm 
fn help(arg : &str){
    match arg {
        "" => println!("Test"),
        "math" => println!("Mathematics fuction, available flags: \n -f : function flag, enable the function parser for determining a fucntion [NOT IMPLEMENTED] 
                            \n -s : sin flag, for computing the respective sine of some angle [NOT IMPLEMENTED] 
                            \n -c : cos flag, for computing the respective cosine of some angle [NOT IMPLEMENTED] 
                            \n -m : memory flag, for saving the result of all following operations into a specified register,"),
        "exit" => println!("exit the user process"),
        _ => println!("Invalid argument")
    }
}

/// ## Math Function
/// Takes user input, calls handler of the math module and prints the result of the operations
fn math(arg : &str) {
    match arg {
        "" => {
            let input = take_userm_input();
            let r = math_handler(input.op.as_str().trim(), input.a, input.b);
            let r = {match r {
                Ok(num) => num,
                Err(meg) => {println!("{meg}");return}
            }};
            println!("{} {} {} = {}", input.a , input.op, input.b, r)
        }
        _ => println!("Invalid argument")
    }
}

/// ## Take User Math input 
/// it returns a User Math input Struct containing numbers a , b and some string
fn take_userm_input() -> commons::UMinput {
    println!("Input a:");
    let mut tx = String::new();
    io::stdin().read_line(&mut tx).unwrap();
    let x : i64 = {match tx.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    }};
    println!("Input b:");
    let mut ty = String::new();
    io::stdin().read_line(&mut ty).unwrap();
    let y : i64 = {match ty.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    }};
    println!("Input operation (+ , - , * , /)");
    let mut tz = String::new();
    io::stdin().read_line(&mut tz).unwrap();
    let z = tz; 
    let g= commons::UMinput { a: (x), b: (y) , op : {z}};
    g
}