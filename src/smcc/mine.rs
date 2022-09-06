use nannou::prelude::*;

const N_POINTS: i32 = 100;

pub fn mine(draw: &Draw) {
    let radius = 150.0;
    // let points = (0..=360).step_by(72).map(|i| {
    //     let radian = deg_to_rad(i as f32);
    //     let x = radian.sin() * radius;
    //     let y = radian.cos() * radius;
    //     pt2(x, y)
    // });
    let points = (0..N_POINTS).map(|k| {
        let t = 2.0 * PI * k as f32 / N_POINTS as f32;
        let x = 16.0 * t.sin().powi(3);
        let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
        let point = pt2(x, y) * 12.0;
        (point, srgb8(248, 24, 24))
    });

    draw.polygon().points_colored(points);
    // draw.polygon().color(STEELBLUE).points(points);

    // 自前のコード
    // for i in -100..100 {
    //     for j in -100..100 {
    //         for h in 0..100 {
    //             let x = h as f32;
    //             let y = h as f32;
    //             let radiusLine = pt2(x.cos(), y.sin()) * 50.0;
    //             draw.rect()
    //                 .x_y(radiusLine.x, radiusLine.y)
    //                 .w_h(1.0, 1.0)
    //                 .color(PURPLE);

    //             // if radiusLine.x.abs() > i.abs() as f32 && radiusLine.y.abs() > j.abs() as f32 {
    //             //     draw.rect()
    //             //         .x_y(i as f32, j as f32)
    //             //         .w_h(1.0, 1.0)
    //             //         .color(BLUE);
    //             // }

    //             draw.rect().x_y(i as f32, 0.0).w_h(1.0, 1.0).color(RED);
    //             draw.rect().x_y(0.0, j as f32).w_h(1.0, 1.0).color(RED);
    //         }
    //     }
    // }
}
