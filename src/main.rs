mod smcc;
use crate::smcc::mine::mine;
use nannou::prelude::*;

static RADIUS: f32 = 50.0;
fn main() {
    // nannou::app(model).update(update).simple_window(view).run()
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

    mine(app);

    //理想(mackeee)
    // let boundary = app.window_rect();
    // for y in -50..=50 {
    //     for x in -50..=50 {
    //         let dx = x as f32 - 0.0;
    //         let dy = y as f32 - 0.0;
    //         let dist = (dx.powf(2.0) + dy.powf(2.0)).sqrt();

    //         if dist < RADIUS {
    //             draw.rect()
    //                 .x_y(x as f32, y as f32)
    //                 .w_h(1.0, 1.0)
    //                 .color(BLACK);
    //         }
    //     }
    // }

    draw.to_frame(app, &frame).unwrap()
}
