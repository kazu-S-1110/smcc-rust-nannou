use nannou::prelude::*;

fn main() {
    // nannou::app(model).update(update).simple_window(view).run()
    nannou::sketch(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);
    draw.to_frame(app, &frame).unwrap()
}
