use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;

const WIDTH: u32 = 512;
const TIMESCALE: f64 = 0.5;

const STEERINGFORCE: f32 = 0.4;
const DRIVERSPEED: f32 = 0.015;

// set to 0 to never fade/erase anything
const FADING: f32 = 0.04;

#[derive(Debug)]
struct Model {
    noise: Perlin,
    noise_based_pos: [f32; 2],

    driver_pos: [f32; 2],
    driver_direction: f32,
}

fn main() {
    nannou::app(setup).update(update).run();
}

fn setup(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WIDTH, WIDTH)
        .view(view)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        noise: Perlin::new().set_seed(11223344),
        noise_based_pos: [0.0, 0.0],

        driver_direction: 1.0,
        driver_pos: [0.0, 0.0],
    }
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn update(app: &App, model: &mut Model, _update: Update) {
    let noise_x = model
        .noise
        .get([app.time as f64 * TIMESCALE, app.time as f64]) as f32;
    let noise_y = model
        .noise
        .get([app.time as f64, app.time as f64 * TIMESCALE]) as f32;

    model.noise_based_pos = [noise_x, noise_y];

    model.driver_direction += (random_f32() * 2.0 - 1.0) * STEERINGFORCE;
    let pos = model.driver_pos;

    let x = wrap_coord(pos[0] + model.driver_direction.sin() * DRIVERSPEED);
    let y = wrap_coord(pos[1] + model.driver_direction.cos() * DRIVERSPEED);

    model.driver_pos = [x, y];
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    if app.elapsed_frames() == 0 {
        draw.background().color(BLACK);
    } else {
        let w = app.main_window().inner_size_pixels().0 as f32;
        let h = app.main_window().inner_size_pixels().1 as f32;
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(w, h)
            .color(rgba(0.0, 0.0, 0.0, FADING));
    }

    let noise1 = model
        .noise
        .get([1.0, 1.0, app.time as f64 * 2.0 * TIMESCALE]) as f32;
    let noise2 = model
        .noise
        .get([1.0, app.time as f64 * 2.0 * TIMESCALE, 1.0]) as f32;

    let mut pos = model.noise_based_pos;
    draw.ellipse()
        .radius(5.0 + noise1 * 5.0)
        .x_y(normalized_to_window(pos[0]), normalized_to_window(pos[1]))
        .color(rgb(pos[0] * 0.5 + 0.5, pos[1] * 0.5 + 0.5, noise2));

    pos = model.driver_pos;
    draw.ellipse()
        .radius(5.0 + noise2 * 5.0)
        .x_y(normalized_to_window(pos[0]), normalized_to_window(pos[1]))
        .color(rgb(pos[0] * 0.5 + 0.5, pos[1] * 0.5 + 0.5, 1.0 - noise1));

    draw.to_frame(app, &frame).unwrap();
}

fn wrap_coord(coord: f32) -> f32 {
    return if coord > 1.0 {
        coord - 2.0
    } else if coord < -1.0 {
        coord + 2.0
    } else {
        coord
    };
}

fn normalized_to_window(coord: f32) -> f32 {
    return map_range(coord, -1.0, 1.0, WIDTH as f32 / -2.0, WIDTH as f32 / 2.0);
}