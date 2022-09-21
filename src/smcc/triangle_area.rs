pub fn triangle_area(line_ab: i32, line_cd: i32, line_bc: i32) -> i32 {
    let be_ratio = line_ab * line_cd;
    let bd_ratio = line_ab + line_cd;
    let height = (be_ratio as f32) / (bd_ratio as f32);
    return (line_bc as f32 * height / 2.0) as i32;
}
