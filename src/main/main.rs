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
use std::time::{Duration};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::algorithms::{golf};
use crate::graphics::draw_field;
use crate::input::{field_from_name, get_input_file};
use crate::output::get_output_file;

pub fn main() {
    let name_input = get_input_file();
    let mut name_output = get_output_file();

    //create the window and launch the algorithm
    rendering(&*name_input, &mut name_output);
}

///
/// Function creating the window and launching the algorithm
///
fn rendering(input: &str, out : &mut String) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("golf Project", 800, 600).build().unwrap();

    //window's creation
    let mut canvas: Canvas<Window> = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    //draw the background
    graphics::background(&mut canvas);

    //launch the algorithm
    instance(&mut canvas, input, out);

    //update canvas
    canvas.present();

    //event listener, waiting for the user to press esc or to close the window
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


///
/// Function launching the algorithm on the file input0.txt which is deprecated now that you can specify the input in the main program
///
#[allow(dead_code)]
fn instance_0(canvas: &mut Canvas<Window>) {
    //parse input
    let mut field = field_from_name("input0").expect("Cannot load inputs field.");
    //draw field
    draw_field(canvas, &field);

    //output0.txt as output
    let output_path = format!(
        "{}{}{}",
        current_dir().unwrap().to_str().unwrap(), "\\output\\", "output0.txt"
    );
    //check if output already exists
    match fs::remove_file(&output_path) {
        Ok(_response) => println!("File {:?} existed already so it has been erased before use", &output_path),
        Err(_error) => ()
    };

    //run algorithm
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

///
/// Function launching the algorithm for the input and output specified
///

fn instance(canvas: &mut Canvas<Window>, name_input: &str, name_output: &mut String) {
    //check if output already exists
    let output_path = format!(
        "{}{}{}.{}",
        current_dir().unwrap().to_str().unwrap(), "\\output\\", name_output, "txt"
    );
    match fs::remove_file(&output_path) {
        Ok(_response) => println!("File {:?} existed already so it has been erased before use", &output_path),
        Err(_error) => ()
    };

    //parse input
    let mut field = field_from_name(name_input).expect("Cannot load inputs field.");
    //draw field
    draw_field(canvas, &field);
    //add extension to the output
    name_output.push_str(".txt");
    //run algo

    //let now = Instant::now();
    golf(canvas, &mut field.balls, &mut field.holes, &name_output);
    //let time = now.elapsed().as_millis();

}

///
/// Function simulating the launch of the algorithm on the file input1.txt
///
#[allow(dead_code)]
fn instance_demo(canvas: &mut Canvas<Window>) {
    //parse input
    let field = field_from_name("input1").expect("Cannot load inputs field.");

    //draw field
    draw_field(canvas, &field);

    //draw lines
    graphics::draw_segment(canvas, &[100, 100], &[400, 300], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[280, 280], &[120, 220], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[320, 380], &[380, 320], Color::RGB(0, 0, 0));
    graphics::draw_segment(canvas, &[480, 220], &[580, 180], Color::RGB(0, 0, 0));
}



