#![allow(non_snake_case)]

mod input;
mod types;
mod graphics;
mod algorithms;
mod output;

use std::env::current_dir;
use std::fs;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::algorithms::{golf};
use crate::graphics::draw_field;
use crate::input::field_from_name;

pub fn main() {
    rendering();
}

fn rendering() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("golf Project", 800, 600).build().unwrap();

// Let's create a Canvas which we will use to draw in our Window
    let mut canvas: Canvas<Window> = window.into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        .build().unwrap();

    graphics::background(&mut canvas);

    instance(&mut canvas);

    //update canvas
    canvas.present();

    //event listener
    let mut event_pump = sdl_context.event_pump().unwrap();
    'pause: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'pause;
                }
                _ => {}
            }
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

#[allow(dead_code)]
fn instance_0(canvas: &mut Canvas<Window>) {
    let mut field = field_from_name("input0").expect("Cannot load inputs field.");
    draw_field(canvas, &field);


    let output_path = format!(
        "{}{}{}",
        current_dir().unwrap().to_str().unwrap(), "\\output\\", "output0.txt"
    );
    match fs::remove_file(&output_path) {
        Ok(_response) => println!("File {:?} existed already so it has been erased before use", &output_path),
        Err(_error) => ()
    };


    golf(canvas, &mut field.balls, &mut field.holes, &String::from("output0.txt"));
}

#[test]
fn should_solve_instance_0() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("golf Project", 800, 600).build().unwrap();

// Let's create a Canvas which we will use to draw in our Window
    let mut canvas: Canvas<Window> = window.into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        .build().unwrap();

    graphics::background(&mut canvas);
    instance_0(&mut canvas);

    //event listener
    let mut event_pump = sdl_context.event_pump().unwrap();
    'pause: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'pause;
                }
                _ => {}
            }
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));


    let contents = fs::read_to_string(format!(
        "{}{}{}",
        current_dir().unwrap().to_str().unwrap(), "\\output\\", "output0.txt"))
        .expect("Something went wrong reading the file");



    assert_eq!(
        format!("{}", contents),
        "[280, 280] to [400, 300]\n[100, 100] to [120, 220]\n"
    )
}

fn instance(canvas: &mut Canvas<Window>) {
    let output_path = format!(
        "{}{}{}",
        current_dir().unwrap().to_str().unwrap(), "\\output\\", "outputs.txt"
    );
    match fs::remove_file(&output_path) {
        Ok(_response) => println!("File {:?} existed already so it has been erased before use", &output_path),
        Err(_error) => ()
    };
    let mut field = field_from_name("inputs").expect("Cannot load inputs field.");

    draw_field(canvas, &field);
    golf(canvas, &mut field.balls, &mut field.holes, &String::from("outputs.txt"));
}

#[allow(dead_code)]
fn instance_demo(canvas: &mut Canvas<Window>) {
    let field = field_from_name("input1").expect("Cannot load inputs field.");


    draw_field(canvas, &field);

    //[((1 , 1), (4 , 3)) , ((2.8 , 2.8), (1.2 , 2.2)) , ((3.2 , 3.8) , (3.8 , 3.2)) , ((4.8 , 2.2) , (5.8 , 1.8))]
    //draw line
    graphics::draw_segment(canvas, &[100, 100], &[400, 300], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[280, 280], &[120, 220], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[320, 380], &[380, 320], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[480, 220], &[580, 180], Color::RGB(0, 0, 0));
}


