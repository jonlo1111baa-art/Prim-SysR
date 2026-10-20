//! #### IO module
//! Provides fuctionality for obtaining input of multiple types
//! included math input, coordinate input, function parser input
//! note that **commons** is the one to provide the required enums and structs

use std::io;
use crate::commons::{self, Twocoord};

/// standard read line fucntion using std::io 
pub fn readline() -> String {
    let mut tx = String::new();
    let x: String;
    match io::stdin().read_line(&mut tx) {
        Ok(num) => x = num.to_string(),
        Err(_) => {println!("Failure to read line"); x = String::new()}
    }
    x
}

/// ## Take User Math input 
/// it returns a User Math input Struct containing numbers a , b and some string
pub fn take_usermoi() -> commons::UMOinput {
    println!("Input a:");
    let x : i64= match readline().trim().parse() {
        Ok(str) => str,
        Err(_) => {println!("Failed to parse"); 0}
    };
    println!("Input b:");
    let y : i64= match readline().trim().parse() {
        Ok(str) => str,
        Err(_) => {println!("Failed to parse"); 0}
    };
    println!("Input operation (+ , - , * , /)");
    let z = readline();
    let g= commons::UMOinput { a: (x), b: (y) , op : {z}};
    g
}

pub fn take_userci() -> commons::Twocoord{
    println!("Input x component");
    let x : i64 = match readline().trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Failed to parse"); 0}
    };
    println!("Input y component");
    let y : i64 = match readline().trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Failed to parse"); 0}
    };
    let c = Twocoord{x : {x}, y : {y}};
    c
}
