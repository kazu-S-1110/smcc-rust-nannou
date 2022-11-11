pub fn reduce_sum(target_num: i32) -> i32 {
    let v = (1..=target_num).collect::<Vec<i32>>();

    v.into_iter().reduce(|x, y| x + y).unwrap()
}
