// Dedicated Mathematics module 
// accepts 1 argument  max and return an err when not given such arg.
// implements integer sum,  difference, times and division
// TO DO: implement a handler module inside the math module to evoid packing in the main handler
// fix unexpected behaviour in the math module


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

fn nan_closer() -> i64 {
   let n : i64;
   let tx : i64 = (2 * 111)/2;
   n = match compute_power(tx , 2) {
       Ok(num) => num, 
       Err(_) => 0
   };
   n 
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