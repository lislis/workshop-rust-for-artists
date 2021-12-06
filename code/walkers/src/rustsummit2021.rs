use nannou::prelude::*;

const WIDTH: f32 = 512.0;
const STEP_SIZE: f32 = 10.0; 

fn main() {
    nannou::app(model).update(update).run();
}
#[derive(Debug, Clone, Copy)]
enum Style {
    Circle,
    Rect
}

#[derive(Debug, Clone, Copy)]
struct Step {
    position: Vec2,
    style: Style,
    size: f32
}

impl Step {
    fn new(position: Vec2, style: Style) -> Self {
        Self {
            position,
            style,
            size: map_range(random_f32(), 0.0, 1.0, 10.0, 30.0)
        }
    }
}

#[derive(Debug)]
struct Walker {
    steps: Vec<Step>
}

impl Walker {
    fn new() -> Self {
        Self {
            steps: vec!()
        }
    }

    fn new_from_pos(position: Vec2, style: Style) -> Self {
        Self {
            steps: vec!(Step::new(position, style))
        }
    }

    fn update(&mut self, style: &Style) {
        let last_step = *self.steps.last()
                                        .unwrap_or(&Step::new(
                                            vec2(0.0, 0.0), 
                                            Style::Circle));
        let random_vec = vec2(
            (random_f32() * (STEP_SIZE * 2.0)) - STEP_SIZE,
            (random_f32() * (STEP_SIZE * 2.0)) - STEP_SIZE
        );
        let new_position_vec = last_step.position + random_vec;
        let new_step = Step::new(
            new_position_vec, 
            *style);
        self.steps.push(new_step);
    }

    fn follow_mouse(&mut self, mouse_position: Vec2, style: &Style) {
        let last_step = *self.steps.last()
                                        .unwrap_or(&Step::new(
                                            vec2(0.0, 0.0), 
                                            Style::Circle));
        let new_vec = mouse_position - last_step.position;
        let new_vec = new_vec.normalize_or_zero() * STEP_SIZE;
        let new_vec = new_vec * vec2(random_f32(), random_f32());
        let new_pos = last_step.position + new_vec;
        let new_step = Step::new(new_pos, *style);
        self.steps.push(new_step);
    }
}

#[derive(Debug)]
struct Model {
    walkers: Vec<Walker>,
    current_style: Style
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
        walkers: vec!(Walker::new()),
        current_style: Style::Circle
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => model.current_style = Style::Circle,
        Key::Key2 => model.current_style = Style::Rect,
        _ => {}
    }
}

fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            let mouse = app.mouse.position();
            model.walkers.push(Walker::new_from_pos(mouse, model.current_style));
        },
        MouseButton::Right => model.walkers = vec!(),
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    //println!("{:?}", model.walkers.len());
    for walker in model.walkers.iter_mut() {
        //walker.update(&model.current_style);
        walker.follow_mouse(app.mouse.position(), &model.current_style);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.background().color(BLACK);

    for walker in model.walkers.iter() {
        for step in walker.steps.iter() {
            let hue = map_range(step.position.x, - WIDTH/2.0, WIDTH / 2.0, 0.0, 1.0); 
            
            match step.style {
                Style::Circle => {
                    draw.ellipse()
                    .xy(step.position)
                    .w_h(step.size, step.size)
                    .hsla(hue, 0.5, 0.5, 0.1);
                },
                Style::Rect => {
                    draw.rect()
                    .xy(step.position)
                    .w_h(step.size, step.size)
                    .hsla(hue, 0.5, 0.5, 0.1);
                },
            }
            
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
