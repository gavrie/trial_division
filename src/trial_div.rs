pub fn trial_div(mut n: u64) -> Vec<u64> {
    let mut a = vec![];
    let mut f = 2;
    while n > 1 {
        if n % f == 0 {
            a.push(f);
            n /= f;
        } else {
            f += 1;
        }
    }
    a
}
