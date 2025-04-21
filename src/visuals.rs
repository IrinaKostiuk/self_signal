use nannou::prelude::*;
use crate::system::Model;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Fading trail background for persistence
    draw.background().color(srgba(0.0, 0.0, 0.0, 0.05));

    // Center-pulsing circle
    let radius = 100.0 + model.color_phase * 50.0;
    let hue = (model.time * 0.05) % 1.0;
    let color = hsv(hue, 0.8, 0.9);

    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(radius)
        .color(color);

    draw.to_frame(app, &frame).unwrap();
}