use nannou::prelude::*;
use rand::prelude::*;

const WIDTH: f32 = 512.0;

#[derive(Debug)]
struct Model {
    positions: Vec<Vec2>
}

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

    let x  = random::<f32>() * WIDTH;
    let y = random::<f32>() * WIDTH;
    Model {
        positions: vec!(vec2(x, y))
    }
}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let last_point = model.positions.last();
    
    if let Some(last) = last_point {
        let last = *last;
        let x = last.x + (random::<f32>() * 20.0 - 10.0);
        let y = last.y + (random::<f32>() * 20.0 - 10.0);
        model.positions.push(vec2(x, y));
    }    
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let topleft = draw
        .translate(vec3(WIDTH / -2.0, WIDTH / 2.0, 0.0))
        .scale_y(-1.0);

    for position in model.positions.iter() {
        let hue = map_range(position.x, 
            0.0, WIDTH,
            0.0, 1.0);

        topleft
            .ellipse()
            .xy(*position)
            .wh(vec2(10.0, 10.0))
            .hsla(hue, 0.5,0.5, 1.0);
    }

    draw.to_frame(app, &frame).unwrap();
}
