pub fn min_cents_num(target_num: u32) {
    let v_quarter = 25;
    let v_dime: u32 = 10;
    let v_nickel: u32 = 5;
    let v_pennie: u32 = 1;

    let mut rest = target_num;

    println!("target:{}", target_num);
    if rest / v_quarter > 0 {
        rest %= v_quarter;
        println!("quarter: {}", rest / v_quarter)
    } else {
        println!("quarter: 0",)
    }

    if rest / v_dime > 0 {
        println!("dime: {}", rest / v_dime);
        rest %= v_dime;
    } else {
        println!("dime: 0");
    }

    if rest / v_nickel > 0 {
        println!("nickel: {}", rest / v_nickel);
        rest %= v_nickel;
    } else {
        println!("nickel: 0");
    }

    if rest / v_pennie > 0 {
        println!("pennie: {}", rest / v_pennie);
    }

    println!("----")
}
