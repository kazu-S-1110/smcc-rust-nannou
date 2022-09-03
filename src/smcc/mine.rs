use nannou::prelude::*;

pub fn mine(app: &App) {
    let draw = app.draw();
    // 自前のコード
    for i in -100..100 {
        for j in -100..100 {
            for h in 0..100 {
                let x = h as f32;
                let y = h as f32;
                let radiusLine = pt2(x.cos(), y.sin()) * 50.0;
                draw.rect()
                    .x_y(radiusLine.x, radiusLine.y)
                    .w_h(1.0, 1.0)
                    .color(PURPLE);

                // if radiusLine.x.abs() > i.abs() as f32 && radiusLine.y.abs() > j.abs() as f32 {
                //     draw.rect()
                //         .x_y(i as f32, j as f32)
                //         .w_h(1.0, 1.0)
                //         .color(BLUE);
                // }

                draw.rect().x_y(i as f32, 0.0).w_h(1.0, 1.0).color(RED);
                draw.rect().x_y(0.0, j as f32).w_h(1.0, 1.0).color(RED);
            }
        }
    }
}
