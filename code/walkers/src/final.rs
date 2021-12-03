use nannou::prelude::*;

const WIDTH: f32 = 512.0;
const STEP_SIZE: f32 = 10.0;

#[derive(Debug, Clone, Copy)]
enum Viz {
    Dots,
    Circles,
    Rects
}

#[derive(Debug, Clone)]
struct Walker {
    steps: Vec<(Vec2, f32)>,
    style: Viz
}

impl Walker {
    pub fn new(x: f32, y: f32, style: Viz) -> Self {
        Self {
            steps: vec!((vec2(x, y), (random_f32() * 30.0) + 5.0)),
            style
        }
    }

    pub fn update(&mut self) {
        let last_step = self.steps.last().expect("No last step");
        let random_vec = vec2(
                                (random_f32() * (STEP_SIZE * 2.0)) -STEP_SIZE, 
                                (random_f32() * (STEP_SIZE * 2.0)) -STEP_SIZE);
        let new_step = last_step.0 + random_vec;
        let clamp_min = Vec2::new(- WIDTH / 2.0, - WIDTH / 2.0);
        let clamp_max = Vec2::new(WIDTH / 2.0, WIDTH / 2.0);
        let new_step = new_step.clamp(clamp_min, clamp_max);
        self.steps.push((new_step, (random_f32() * 30.0) + 5.0));
    }
}
#[derive(Debug)]
struct Model {
    walkers: Vec<Walker>,
    style: Viz
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

    Model {
        walkers: vec!(Walker::new(0.0, 0.0, Viz::Circles)),
        style: Viz::Circles
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => model.style = Viz::Dots,
        Key::Key2 => model.style = Viz::Circles,
        Key::Key3 => model.style = Viz::Rects,
        _ => {}
    }
}

fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            let mouse_pos = app.mouse.position();
            model.walkers.push(Walker::new(mouse_pos.x, mouse_pos.y, model.style));
        },
        MouseButton::Right => {
            model.walkers = vec!()
        },
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for walker in model.walkers.iter_mut() {
        walker.update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for walker in model.walkers.iter() {
        for step in walker.steps.iter().step_by(3) {
            let hue = map_range(step.0.x, 0.0, WIDTH, 0.0, 1.0);
            match walker.style {
                Viz::Dots => {
                    draw.ellipse()
                        .w_h(2.0, 2.0)
                        .xy(step.0)
                        .hsla(hue, 0.5, 0.5, 0.1);
                },
                Viz::Circles => {
                    draw.ellipse()
                        .w_h(step.1, step.1)  
                        .xy(step.0)
                        .hsla(hue, 0.5,0.5, 0.05);
                },
                Viz::Rects => {
                    draw.rect()
                    .w_h(step.1, step.1)
                    .xy(step.0)
                    .hsla(hue, 0.5,0.5, 0.05);
                },
            }
        }
    }
    
    draw.to_frame(app, &frame).unwrap();
}
