// The index.html of the soul
mod audio;
mod visuals;
mod input;
mod system;

use nannou::prelude::*;
use system::Model;

fn main() {
    nannou::app(model)
        .update(system::update)
        .view(visuals::view)
        .event(input::event)
        .run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800, 800)
        .title("SELF//SIGNAL")
        .build()
        .unwrap();

    let stream = audio::play_background_audio();

    Model::new(app, window_id, stream)
}
