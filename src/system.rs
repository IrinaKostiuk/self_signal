use nannou::prelude::*;
use cpal::Stream;

pub struct Model {
    pub window_id: WindowId,
    pub time: f32,
    pub color_phase: f32, // A sin-wave modulated value — used for pulsating visuals (like the inner light 💡)
    pub audio_state: f32, 
    pub interaction_state: f32,
    pub _audio_stream: Option<Stream>, 
}

impl Model {
    pub fn new(_app: &App, window_id: WindowId, stream: Stream) -> Self {
        Model {
            window_id,
            time: 0.0,
            color_phase: 0.0,
            audio_state: 0.0,
            interaction_state: 0.0,
            _audio_stream: Some(stream),
        }
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.02; // animate time
    model.color_phase = (model.time * 0.5).sin(); // controls visual color
}