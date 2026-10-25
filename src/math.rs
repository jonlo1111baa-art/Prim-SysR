//! ### Math
//! 
//! Dedicated Mathematics module 
//! accepts 1 argument  max and return an err when not given such arg.
//! implements integer sum,  difference, times and division

use crate::commons;

pub struct MathDesc{}
impl MathDesc {
    pub fn build_desc () -> String{
        let tx = String::from("Mathematics fuction, available flags: \n -f : function flag, enable the function parser for determining a fucntion [NOT IMPLEMENTED] 
                            \n -s : sin flag, for computing the respective sine of some angle [NOT IMPLEMENTED] 
                            \n -c : cos flag, for computing the respective cosine of some angle [NOT IMPLEMENTED] 
                            \n -m : memory flag, for saving the result of all following operations into a specified register,");
        tx
    }
}

pub fn math_handler(op : &str, a : i64 , b : i64) -> Result<i64 , &'static str > {
    let r: i64;
    match op {
        "+" => r = compute_sum(a , b),
        "-" => r = compute_difference(a, b),
        "*" => r = compute_times(a, b),
        "/" => r = compute_int_divisor(a, b).unwrap(),
        "**" => r = compute_power(a, b).unwrap(),
        _ => r = 0
    }
    Ok(r)
}

fn compute_sum(a : i64 , b : i64) -> i64 {
    let r = a + b;
    r
}

fn compute_difference(minuend : i64 , subtrahend : i64) -> i64 {
    let r = minuend - subtrahend;
    r
}

fn compute_times(a : i64 , b : i64 ) -> i64 {
    let r = a * b;   
    r
}

fn compute_int_divisor(dividend : i64  , divisor : i64 ) -> Result<i64,&'static str> {
    if divisor == 0 {
        Err("Division by zero is undefined")
    }
    else {
        let r = dividend/divisor;
        Ok(r)
    }
}

/// #### Linear interpolation fucntion
/// Takes 2 coordinates c1, c2
/// finds the slope of the linear polinomial wich crosses them
/// then eveluates such polynomial at x
pub fn lerp(c1 : &commons::Twocoord, c2 : &commons::Twocoord, x : i64) -> Option<i64> {
    if x == 0{
        None
    }
    else{
        let r = (x*c2.y - x*c1.y - c1.x*c2.y + c1.x*c1.y)/(c2.x);
        Some(r)
    }
}

fn compute_power(a : i64,b : i64) -> Result<i64, &'static str> {
    let mut tx : i64;
    let mut i : i64 = 1;
    if b == 1 {
        return Ok(a);
    }
    else if b == 0 {
        return Ok(0);
    }
    else if a == 0 {
        return Ok(0);
    }
    else if a == 1 {
        return Ok(1);
    }
    else {
        tx = a * a;
        i = i + 1;
    }
    while i > b{
        tx = tx * tx;
        i = i + 1
    }
    Ok(tx)
}