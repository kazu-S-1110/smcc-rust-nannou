mod smcc;
use crate::smcc::mine::mine;
use crate::smcc::question::question;
use nannou::prelude::*;

fn main() {
    // nannou::app(model).update(update).simple_window(view).run()
    nannou::sketch(view).size(100, 100).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}
static RADIUS: i32 = 25;

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // mine(&draw);
    // question(app);

    //理想(mackeee)
    // for y in -(RADIUS as i32)..=(RADIUS as i32) {
    //     for x in -(RADIUS as i32)..=(RADIUS as i32) {
    //         let dx = x as f32 - 0.0;
    //         let dy = y as f32 - 0.0;
    //         let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
    //         let dd = abs(dist - RADIUS);
    //         if dd < (LINE_WEIGHT / 2.0) {
    //             let opacity = LINE_WEIGHT - dd;
    //             draw.rect().x_y(x as f32, y as f32).w_h(1.0, 1.0).color(rgba(0.0, 0.0, 0.0, opacity));
    //         }
    //     }
    // }

    for y in -25..=25 {
        for x in -25..=25 {
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
