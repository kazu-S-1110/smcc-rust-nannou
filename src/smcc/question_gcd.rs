pub fn question_gcd(target: &str) {
    let mut n: u32 = 0;
    let mut d: u32 = 0;
    for i in 0..target.len() {
        let mut j = target.chars().nth(i);

        if let Some(v) = j {
            if v == '/' {
                n = (&target[0..i]).parse().unwrap();
                d = (&target[i + 1..target.len()]).parse().unwrap();
            }
        }
    }

    // println!("n is {}", n);
    // println!("d is {}", d);

    let g = gcd_fn(n, d);

    // println!("g is {}", g);
    let ans_n = n / g;
    let ans_d = d / g;
    println!("{}/{}", ans_n, ans_d)
}

fn gcd_fn(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd_fn(b, a % b)
    }
}
