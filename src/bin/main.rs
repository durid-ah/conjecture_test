use num::bigint::{ToBigInt, BigInt};
use num::traits::One;
use conjecture_test::ThreadPool;

/// Binomial coefficient
fn n_c_k(n: u128, k: u128) -> BigInt {
    let mut res = BigInt::one();
    for i in 0..k {
        res = (res * (n - i).to_bigint().unwrap()) /
              (i + 1).to_bigint().unwrap();
    }

    res
}

/// Count the true values in the binary representation of `num`
fn count_1s(num: u128) -> u128 {
    let b = format!("{:b}", num);
    let count = b.matches("1").count() as u128;
    count
}

/// Count the number of times `num` is divisible by 2
fn count_factors_of_2(num: BigInt) -> u128 {
    let mut count: u128 = 0;
    let mut div_num = num.clone();
    while (is_even(&div_num)) && (div_num != 0.to_bigint().unwrap()) {
        count += 1;
        div_num = div_num / (2 as u128);
    }

    count
}


fn is_even(num: &BigInt) -> bool {
    return num.clone() % 2.to_bigint().unwrap() == 0.to_bigint().unwrap();
}


fn test_num(num: u128) -> bool {
    let choose = n_c_k(2 * num, num);
    let factors_of_2 = count_factors_of_2(choose);
    factors_of_2 == count_1s(num)
}


fn process_batch(begin: u128, end:u128) {
    println!("Starting batch {}-{}", begin, end);
    for k in begin..end {
        let res = test_num(k);
        if !res {
            break
        }
    }
    println!("Processed batch {}-{}", begin, end);
}


fn main() {
    let pool = ThreadPool::new(3);
    let mut begin: u128 = 1;
    let mut end: u128 =  1 + 10000;
    
    loop {
        pool.execute(move || {
            process_batch(begin, end);
        });
        begin = end;
        end = if (u128::MAX - end) >= 10000  { end + 10000 } else { u128::MAX };
    }
}
