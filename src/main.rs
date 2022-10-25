use std::env;

mod smcc;

fn main() {
    println!("{}", smcc::max_diff::max_diff([1, 3, 5, 7, 78].to_vec()));
    println!("{}", smcc::max_diff::max_diff([3, 1, 4].to_vec()));
    println!(
        "{}",
        smcc::max_diff::max_diff([1, 1, 1, 1000000000, 1000000000].to_vec())
    );
}
