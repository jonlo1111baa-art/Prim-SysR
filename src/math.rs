// Dedicated Mathematics module 
// accepts 1 argument  max and return an err when not given such arg.
// implements integer sum,  difference, times and incomplete division
// note: compute-divisor. 

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

pub fn computeintdivisor(dividend : i64  , divisor : i64 ) -> i64 {

    if dividend > divisor {
        0
    }
    else {
        let c:i64 = 0;
        while dividend > divisor {
            let dividend = dividend - divisor;
            let c = c + 1; 
        }  
        c
    }
}