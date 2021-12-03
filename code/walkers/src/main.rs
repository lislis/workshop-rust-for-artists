use nannou::prelude::*;

const WIDTH: f32 = 512.0;

#[derive(Debug)]
struct Model {}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH as u32, WIDTH as u32)
        .view(view)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {}
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
