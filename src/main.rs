extern crate nannou;
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(512, 512).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    if app.elapsed_frames() % 60 == 0 {
        draw_pyramid(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_pyramid(draw: &Draw) {
    let background_color = random_rgb();
    let tri_face_one_color = random_rgb();
    let tri_face_two_color = random_rgb();

    // Draw a pyrmaid with 2d shapes
    // Draw in perspective
    // Use two triangles
    draw.translate(vec3(-75.0, -15.0, 0.0))
        .tri()
        .points(vec2(0.0, 0.0), vec2(100.0, -25.0), vec2(65.0, 100.0))
        .color(tri_face_one_color);
    draw.translate(vec3(-75.0, -15.0, 0.0))
        .tri()
        .points(vec2(100.0, -25.0), vec2(145.0, 35.0), vec2(65.0, 100.0))
        .color(tri_face_two_color);

    draw.background().color(background_color);
}

fn random_rgb() -> Rgb {
    let r = random_f32();
    let g = random_f32();
    let b = random_f32();
    rgb(r, g, b)
}
