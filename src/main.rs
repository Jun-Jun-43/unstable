use nannou::lyon;
use nannou::lyon::algorithms::hatching::{
    HatchSegment, Hatcher, HatchingOptions, RegularHatchingPattern,
};
use nannou::lyon::algorithms::path::PathSlice;
use nannou::lyon::geom::LineSegment;
use nannou::lyon::path::traits::PathBuilder;
use nannou::lyon::path::Path;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Message {
    text_path: Path,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

impl Message {
    fn new(text_path: Path) -> Self {
        Message {
            text_path,
            velocity: vec2(0.0, 0.0),
            acceleration: vec2(1.0, 0.0),
            mass: 10.0,
        }
    }

    fn apply_force(&mut self, force: Vec2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;

        // self.text_path.iter().enumerate().for_each(|(_i, p)| {
        //     p.to().x += self.velocity.x;
        //     p.to().y += self.velocity.y;
        // });

        self.acceleration *= 0.0;
    }
}

struct CutLine {
    start: Vec2,
    end: Vec2,
}

impl CutLine {
    fn new(win_rect: &Rect) -> Self {
        let start = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.bottom(), win_rect.top()),
        );
        let end = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.bottom(), win_rect.top()),
        );

        CutLine { start, end }
    }

    fn update(&mut self, win_rect: &Rect) {
        self.start = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.bottom(), win_rect.top()),
        );
        self.end = vec2(
            random_range(win_rect.left(), win_rect.right()),
            random_range(win_rect.bottom(), win_rect.top()),
        );
    }
}

struct Model {
    message: Message,
    cut_line: CutLine,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(720, 1280).view(view).build().unwrap();

    let text_message = "Unstable".to_string();

    let win_rect = app.main_window().rect();
    let text_builder = text(text_message.as_str())
        .font_size(150)
        .center_justify()
        .build(win_rect);

    let mut builder = lyon::path::Path::builder();
    for e in text_builder.path_events() {
        builder.path_event(e);
    }
    let path = builder.build();

    let hatching_path = hatching_and_dotted(path.as_slice());

    let message = Message::new(hatching_path);
    let cut_line = CutLine::new(&win_rect);

    Model {
        message,
        cut_line,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window_rect();

    let wind = vec2(random_range(-5.0, 5.0), random_range(-5.0, 5.0));

    model.message.apply_force(wind);
    model.message.update();
    model.cut_line.update(&win_rect);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgba(
        13.0 / 255.0,
        13.0 / 255.0,
        13.0 / 255.0,
        200.0 / 255.0,
    ));

    model
        .message
        .text_path
        .iter()
        .enumerate()
        .for_each(|(i, p)| {
            let noise = Perlin::new();
            let n = noise.get([random_range(-10.0, 10.0), random_range(-10.0, 10.0)]) as f32;
            let l = 10.0 / n;
            draw.line()
                .start(pt2(p.to().x - l, p.to().y + l))
                .end(pt2(p.to().x + l, p.to().y + l))
                .xy(model.message.velocity)
                .rgba(242.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0, 25.0 / 255.0);

            if i % 2 == 0 {
                draw.ellipse()
                    .x_y(
                        p.to().x + model.message.velocity.x,
                        p.to().y + model.message.velocity.y,
                    )
                    .radius(map_range(
                        (i as f32 * 0.05 + app.time * 8.6).sin(),
                        -1.0,
                        1.0,
                        2.0,
                        6.0,
                    ))
                    .rgba(242.0 / 255.0, 159.0 / 255.0, 5.0 / 255.0, 95.0 / 255.0);
            }
        });

    draw.line()
        .start(model.cut_line.start)
        .end(model.cut_line.end)
        .stroke_weight(3.5)
        .color(rgba(
            13.0 / 255.0,
            13.0 / 255.0,
            13.0 / 255.0,
            200.0 / 255.0,
        ));

    draw.to_frame(app, &frame).unwrap();

    if app.elapsed_frames().to_usize().unwrap() <= 450 {
        let file_path = capture_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    } else if app.elapsed_frames().to_usize().unwrap() > 451 {
        println!("end!!")
    }
}

fn hatching_and_dotted(path: PathSlice) -> Path {
    let mut hatches = Path::builder();
    let mut hatcher = Hatcher::new();
    let option = HatchingOptions::default();

    hatcher.hatch_path(
        path.iter(),
        &option,
        &mut RegularHatchingPattern {
            interval: 1.0,
            callback: &mut |segment: &HatchSegment| {
                hatches.add_line_segment(&LineSegment {
                    from: segment.a.position,
                    to: segment.b.position,
                });
            },
        },
    );

    hatches.build()
}

fn capture_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}
