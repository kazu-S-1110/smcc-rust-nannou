use nannou::prelude::*;
use nannou::App;

pub fn question(app: &App) {
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    draw.background().color(BLACK);

    // n_pointを変えることでn角の図形が描ける
    let n_points = 4;
    let radius = win.w().min(win.h()) * 0.1;
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        // ここのTAUは2πのこと
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });

    let another_points = pt2(t % TAU - 2.0, t.sin()) * 100.0;

    let center_points = {
        let x = radius * (TAU * t).cos();
        let y = radius * (TAU * t).sin();
        pt2(x, y)
    };
    //　円運動する
    draw.polygon()
        .color(WHITE)
        .x_y(center_points[0], center_points[1])
        .points(points.clone());
    // 上下に動く
    draw.polygon()
        .color(RED)
        .x_y(0.0, center_points[1])
        .points(points.clone());

    //　波形に動く
    draw.polygon()
        .color(BLUE)
        .x_y(another_points[0], another_points[1])
        .points(points.clone());

    // 回転する
    draw.polygon().color(YELLOW).rotate(t).points(points);
}
