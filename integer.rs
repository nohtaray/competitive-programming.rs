/// n の約数
fn get_divisors(n: i32) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    let sqrt = f64::from(n).sqrt().floor() as i32;
    for i in 1..sqrt {
        if n % i == 0 {
            ret.push(i);
            ret.push(n / i);
        }
    }
    if sqrt * sqrt == n {
        ret.push(sqrt);
    }
    ret
}

/// 素因数分解
fn get_factors(n: i32) -> Vec<i32> {
    let mut n = n;
    if n <= 1 {
        return vec![];
    }
    let mut ret = vec![];
    while n > 2 && n % 2 == 0 {
        ret.push(2);
        n /= 2;
    }

    let mut i = 3;
    while i <= f64::from(n).sqrt() as i32 {
        if n % i == 0 {
            ret.push(i);
            n /= i;
        } else {
            i += 2;
        }
    }
    ret.push(n);
    ret
}

/// x ** y % z
fn mod_pow(x: i64, y: i64, z: i64) -> i64 {
    let mut ret = 1;
    let mut x = x;
    let mut y = y;
    while y > 0 {
        if y & 1 == 1 {
            ret = ret * x % z;
        }
        x = x * x % z;
        y >>= 1;
    }
    ret
}
