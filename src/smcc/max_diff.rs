pub fn max_diff(num_arr: Vec<i32>) -> i32 {
    let max = num_arr.iter().max().unwrap();
    let min = num_arr.iter().min().unwrap();

    max - min
}

pub fn max_exclude_one(num_arr: Vec<i32>) -> i32 {
    let mut total = 0;
    for i in &num_arr {
        total += i
    }
    total - num_arr.iter().min().unwrap()
}
