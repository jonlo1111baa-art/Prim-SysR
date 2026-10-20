//! # PRIM-SYS-R
//! 
//! Prim-Sys-r is the rust port of the now deprecated Prim-Sys, originaly written in C#,
//! Prim-Sys-r is Prim-Sys rewriten and expanded in the Rust programming lenguaje taken advantage of its low-level features
//!
//! this main crate does the following:
//! 
//!  -Taking user input  
//!  -Realizing the main execution loop
//!  -Calling handlers of other crates
//!  -Realizing some default operations

use crate::math::math_handler;
mod math;
mod commons;
mod io;

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
        let command = io::readline();
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
            let input = io::take_usermoi();
            let r = math_handler(input.op.as_str().trim(), input.a, input.b);
            let r = {match r {
                Ok(num) => num,
                Err(meg) => {println!("{meg}");return}
            }};
            println!("{} {} {} = {}", input.a , input.op, input.b, r)
        }
        "lerp" => {let c1 = io::take_userci();
            println!("Yet to be implemented");}
        _ => println!("Invalid argument")
    }
}