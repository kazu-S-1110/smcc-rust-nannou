pub fn stairs(target_num: i32) {
    for e in 0..target_num {
        let blank_num = target_num - (e + 1);
        let string_num = target_num - blank_num;
        let mut outputs_string: String = "".to_string();
        for _i in 0..blank_num {
            outputs_string += " ";
        }
        for _j in 0..string_num {
            outputs_string += "#"
        }

        println!("{}", outputs_string)
    }
}
