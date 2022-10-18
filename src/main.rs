mod smcc;

// use nannou::prelude::*;

fn main() {
    // nannou::app(model).update(update).run();
    smcc::question_gcd::question_gcd("2/6");
    smcc::question_gcd::question_gcd("2/7");
    smcc::question_gcd::question_gcd("10/5");
}
//
// struct Model {
//     balls: Vec<Ball>,
// }
//
// struct Ball {
//     position: Point2,
//     velocity: Vec2,
//     acceleration: Vec2,
//     mass: f32,
// }
//
// impl Ball {
//     fn new(m: f32, x: f32, y: f32) -> Self {
//         // Mass is tied to size
//         let mass = m;
//         let position = pt2(x, y);
//         let velocity = vec2(0.0, 0.0);
//         let acceleration = vec2(0.0, 0.0);
//         Ball {
//             position,
//             velocity,
//             acceleration,
//             mass,
//         }
//     }
//
//     fn new_random(app: &App) -> Self {
//         Ball::new(5.0, app.mouse.x, app.mouse.y)
//     }
//
//     fn apply_force(&mut self, force: Vec2) {
//         let f = force / self.mass;
//         self.acceleration += f;
//     }
//
//     fn update(&mut self) {
//         // Velocity changes according to acceleration
//         self.velocity += self.acceleration;
//         // Position changes by velocity
//         self.position += self.velocity;
//         // We must clear acceleration each frame
//         self.acceleration *= 0.0;
//     }
//
//     // Draw Ball
//     fn display(&self, draw: &Draw) {
//         draw.ellipse()
//             .xy(self.position)
//             .w_h(self.mass * 16.0, self.mass * 16.0)
//             .rgba(0.0, 0.0, 255.0, 0.5)
//             .stroke(BLUE)
//             .stroke_weight(2.0);
//     }
//
//     // Bounce off bottom of window
//     fn check_edges(&mut self, rect: Rect) {
//         if self.position.y < rect.bottom() {
//             self.velocity.y *= -0.9; // A little dampening when hitting the bottom
//             self.position.y = rect.bottom();
//         }
//     }
// }
//
// fn model(app: &App) -> Model {
//     let rect = Rect::from_w_h(640.0, 360.0);
//     app.new_window()
//         .size(rect.w() as u32, rect.h() as u32)
//         .mouse_pressed(mouse_pressed)
//         .view(view)
//         .build()
//         .unwrap();
//
//     // Nine moving bodies
//     let balls = (0..1).map(|_| Ball::new_random(app)).collect();
//
//     Model { balls }
// }
//
// fn mouse_pressed(app: &App, m: &mut Model, _button: MouseButton) {
//     for mover in &mut m.balls {
//         *mover = Ball::new_random(app);
//     }
// }
//
// fn update(app: &App, m: &mut Model, _update: Update) {
//     for i in 0..m.balls.len() {
//         // Gravity is scaled by mass here!
//         let gravity = vec2(0.0, -0.1 * m.balls[i].mass);
//
//         // Apply gravity
//         m.balls[i].apply_force(gravity);
//         m.balls[i].update();
//         m.balls[i].check_edges(app.window_rect());
//     }
// }
//
// fn view(app: &App, m: &Model, frame: Frame) {
//     // Begin drawing
//     let draw = app.draw();
//     draw.background().color(WHITE);
//
//     // Draw movers
//     for mover in &m.balls {
//         mover.display(&draw);
//     }
//
//     // Write the result of our drawing to the window's frame.
//     draw.to_frame(app, &frame).unwrap();
// }
