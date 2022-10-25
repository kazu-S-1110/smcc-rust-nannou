pub fn max_diff(num_arr: Vec<i32>) -> i32 {
    let max = num_arr.iter().max().unwrap();
    let min = num_arr.iter().min().unwrap();

    max - min
}
