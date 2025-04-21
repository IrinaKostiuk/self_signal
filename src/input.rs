use nannou::prelude::*;
use crate::system::Model;
use nannou::event::Event;

pub fn event(_app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent { simple, .. } = event {
        match simple {
            Some(KeyPressed(_key)) => {
                model.color_phase += 0.3;
            }
            Some(MouseMoved(pos)) => {
                let speed = pos.x * 0.0005;
                model.time += speed;
            }
            _ => {}
        }
    }
}