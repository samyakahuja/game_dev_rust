mod components;
mod physics;
mod animator;
mod keyboard;
mod renderer;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};

use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

/// used to send events to keyboard system
pub enum MovementCommand {
    Stop,
    Move(Direction),
}

/// map for direction -> row in spritesheet (0 indexed).
fn direction_spritesheet_row(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}

/// create animation frames of given spritesheet index and direction
fn character_animation_frames (
    spritesheet: usize,
    top_left_frame: Rect,
    direction: Direction
) -> Vec<Sprite> {
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet_row(direction);

    let mut frames = Vec::new();
    for i in 0..3 {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x() + frame_width as i32 * i,
                y_offset,
                frame_width,
                frame_height,
            ),
        })
    }

    frames
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("the rust game", 320, 240)
        .position_centered()
        .resizable()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut dispatcher = DispatcherBuilder::new()
        .with(keyboard::Keyboard, "Keyboard", &[])
        .with(physics::Physics, "Physics", &["Keyboard"])
        .with(animator::Animator, "Animator", &["Keyboard"])
        .build();

    let mut world = World::new();
    dispatcher.setup(&mut world.res);
    renderer::SystemData::setup(&mut world.res);

    let movement_command: Option<MovementCommand> = None;
    world.add_resource(movement_command);

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];
    // first texture in textures array
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0, 0, 26, 36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right),
    };

    world.create_entity()
        .with(KeyboardControlled)
        .with(Position(Point::new(0, 0)))
        .with(Velocity {speed: 0, direction: Direction::Right})
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        let mut movement_command = None;
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Left));
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Right));
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Up));
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat:false, .. } => {
                    movement_command = Some(MovementCommand::Move(Direction::Down));
                },
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } |
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    movement_command = Some(MovementCommand::Stop);
                }
                _ => {}
            }
            if let Event::Window { win_event, .. } = event {
                if let WindowEvent::Resized(w, h) = win_event {
                    // TODO: improve upon this! Maybe list certain resolution-scale mappings
                    if (10.0 * w as f32 / h as f32).trunc() as i32 == 13 {
                        let x_scale = w as f32 / 320.0;
                        let y_scale = h as f32 / 240.0;
                        canvas.set_scale(x_scale, y_scale)?;
                    }
                }
            }
        }

        // send movement_command to the keyboard system through a resource
        *world.write_resource() = movement_command;

        // Update
        dispatcher.dispatch(&mut world.res);
        world.maintain();

        // Render
        renderer::render(&mut canvas, Color::RGB(255, 255, 255), &textures, world.system_data())?;

        // Time management (20fps)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
    }

    Ok(())
}
