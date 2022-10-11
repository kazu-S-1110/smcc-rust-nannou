pub fn row_of_number(n: u32, x: u32, y: u32) {
    let mut vect: Vec<u32> = (0..n).collect();

    vect[0] = x;
    vect[1] = y;

    for i in 2..n {
        vect[i as usize] = (vect[(i - 2) as usize] + vect[(i - 1) as usize]);
        // println!("{}",vect[(i - 2) as usize]);
        // println!("{}",vect[(i - 1) as usize]);
    }
    println!("last => {}", vect[(n - 1) as usize])
}
