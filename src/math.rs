// Dedicated Mathematics module 
// accepts 1 argument  max and return an err when not given such arg.
// implements integer sum,  difference, times and incomplete division
// TO DO: implement a handler module inside the math module to evoid packing in the main handler


pub fn computesum(a : i64 , b : i64) -> i64 {
    let r = a + b;
    r
}

pub fn computedifference(minuend : i64 , subtrahend : i64) -> i64 {
    let r = minuend - subtrahend;
    r
}

pub fn computetimes(a : i64 , b : i64 ) -> i64 {
    let r = {
        let mut a = a;
        let mut i = 0;
        while i > (b + 1){
            a = a + a;
            i = i + 1;
        }
        a
    };
    r
}

pub fn computeintdivisor(dividend : i64  , divisor : i64 ) -> Result<i64,&'static str> {
    if divisor == 0 {
        Err("Division by zero is undefined")
    }
    else {
        let r = dividend/divisor;
        Ok(r)
    }
}