use nannou::prelude::*;

static RADIUS: f32 = 25.0;

fn main() {
    nannou::sketch(view).size(100, 100).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    for y in -25..=0 {
        for x in -25..=0 {
            let dx = x as f32 - 0.0;
            let dy = y as f32 - 0.0;
            let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

            let diff = dist - RADIUS as f32;

            let abs_diff = abs(diff);
            match diff {
                -1.0..=1.0 => draw
                    .rect()
                    .x_y(x as f32, y as f32)
                    .w_h(1.0, 1.0)
                    .color(rgb(abs_diff, abs_diff, abs_diff)),
                _ => draw
                    .rect()
                    .x_y(x as f32, y as f32)
                    .w_h(1.0, 1.0)
                    .color(WHITE),
            };
        }
    }

    draw.to_frame(app, &frame).unwrap()
}
