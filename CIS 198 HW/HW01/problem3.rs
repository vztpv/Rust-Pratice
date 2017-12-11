

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    let mut result : Vec<u32> = Vec::new();

    for i in 2..n { // i in (2, n-1)
        let mut is_prime : bool = true;
        if i == 2 {
            result.push(i);
            continue;
        }

        for j in 2..i { // j in (2, i-1) 
            if i % j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            result.push(i);
        }
    }
    
    result
}
