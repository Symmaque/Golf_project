
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn main() {
    rendering();
}

fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, color: Color)
{
    canvas.set_draw_color(color);

    //(x-x0)² + (y-y0)² <= radius²
    //(y-y0)² <= radius² - (x-x0)²
    let x0 = center.x();
    let y0 = center.y();
    let r = radius*radius;


    for x in (x0-radius)..(x0+1){
        for y in y0..(y0 + radius){
            if (x - x0)*(x-x0) + (y-y0)*(y-y0) >= r {
                continue;
            }
            canvas.draw_point(Point::new(x, y));
            canvas.draw_point(Point::new(x, 2*y0-y));
            canvas.draw_point(Point::new(2*x0 - x, y));
            canvas.draw_point(Point::new(2*x0 - x, 2*y0-y));
        }
    }
}

fn draw_segment(canvas: &mut Canvas<Window>, start: Point, end: Point, color: Color){
    canvas.set_draw_color(color);
    canvas.draw_line(start, end);
    canvas.draw_line(Point::new(start.x(), start.y()+1), Point::new(end.x(), end.y()+1));
    canvas.draw_line(Point::new(start.x(), start.y()-1), Point::new(end.x(), end.y()-1));
}

fn rendering (){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Golf Project", 800, 600).build().unwrap();

// Let's create a Canvas which we will use to draw in our Window
    let mut canvas : Canvas<Window> = window.into_canvas()
        .present_vsync() //< this means the screen cannot
        // render faster than your display rate (usually 60Hz or 144Hz)
        .build().unwrap();

// color background
    canvas.set_draw_color(Color::RGB(0, 255, 0));
// background
    canvas.fill_rect(Rect::new(0, 0, 800, 600));

    // draw points
    draw_circle(&mut canvas, Point::new(10, 10), 5, Color::RGB(255, 0, 0));
    draw_circle(&mut canvas, Point::new(20, 20), 5, Color::RGB(0, 0, 255));

    //draw line
    draw_segment(&mut canvas, Point::new(10, 10), Point::new(20, 20), Color::RGB(0,0,0));

    //update canvas
    canvas.present();

    //event listener
    let mut event_pump = sdl_context.event_pump().unwrap();
    'pause: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'pause
                },
                _ => {}
            }
        }
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}
