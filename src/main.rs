use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::time::Duration;

struct Circle {
    c: (f32, f32), // In World Coords
    r: u32,
    vel: (f32, f32),
    color: Color,
}

fn render(canvas: &mut WindowCanvas, circles: &Vec<Circle>) {
    let (width, height) = canvas.output_size().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for circle in circles.iter() {
        canvas
            .filled_circle(
                (circle.c.0 + ((width / 2) as f32)) as i16,
                (circle.c.1 + ((height / 2) as f32)) as i16,
                circle.r as i16,
                circle.color,
            )
            .unwrap();
    }
    canvas.present();
}

fn update(circles: &mut Vec<Circle>, canvas: &WindowCanvas) -> Result<(), String> {
    const GRAVITY: f32 = 300.0;
    for circle in circles.iter_mut() {
        //Gravity
        circle.vel.1 += 1.0 * GRAVITY * 0.016;

        //Updating Position
        circle.c.0 += circle.vel.0 * 0.016;
        circle.c.1 += circle.vel.1 * 0.016;

        //Resolving Collisions
        resolve_collisions(circle, canvas);
    }
    Ok(())
}

fn generate(num: u32, radius: u32) -> Result<Vec<Circle>, String> {
    const PARTICLE_SPACING: f32 = 0.1;
    let mut circles: Vec<Circle> = vec![];

    let particles_per_row = f32::sqrt(num as f32);
    let particles_per_col = (num + 1) as f32 / particles_per_row + 1.0;
    let spacing = (radius as f32) * 2.0 + PARTICLE_SPACING;

    for i in 0..num {
        circles.push(Circle {
            c: (
                ((i as f32) % particles_per_row - particles_per_col / 2.0 + 0.5) * spacing,
                ((i as f32) / particles_per_row - particles_per_col / 2.0 + 0.5) * spacing,
            ),
            r: radius,
            vel: (0.0, 0.0),
            color: Color::RGB(0, 0, 255),
        })
    }
    Ok(circles)
}

fn resolve_collisions(circle: &mut Circle, canvas: &WindowCanvas) {
    const COLLISION_DAMP_FACTOR: f32 = 1.0;
    let (width, height) = canvas.output_size().unwrap();
    let half_bound_size: (f32, f32) = (
        (width / 2 - circle.r) as f32,
        (height / 2 - circle.r) as f32,
    );

    if circle.c.0.abs() > half_bound_size.0 {
        circle.c.0 = half_bound_size.0 * circle.c.0.signum();
        circle.vel.0 *= -1.0 * COLLISION_DAMP_FACTOR;
    }
    if circle.c.1.abs() > half_bound_size.1 {
        circle.c.1 = half_bound_size.1 * circle.c.1.signum();
        circle.vel.1 *= -1.0 * COLLISION_DAMP_FACTOR;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;

    //Generate Objects
    let mut circles = generate(500, 5)?;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        // Update
        update(&mut circles, &canvas)?;
        // Render
        render(&mut canvas, &circles);

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
