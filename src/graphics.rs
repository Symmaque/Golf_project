
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::{graphics, types};
use crate::types::GolfField;

pub(crate) fn background (canvas : &mut Canvas<Window>){
    // color background
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    // background
    canvas.fill_rect(Rect::new(0, 0, 800, 600));
}



pub(crate) fn draw_circle(canvas: &mut Canvas<Window>, center: Point, radius: i32, color: Color)
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

pub(crate) fn draw_segment(canvas: &mut Canvas<Window>, start: &types::Point, end: &types::Point, color: Color){
    canvas.set_draw_color(color);
    canvas.draw_line(Point::new(start[0], start[1]), Point::new(end[0], end[1]));
    canvas.draw_line(Point::new(start[0], start[1]+1), Point::new(end[0], end[1]+1));
    canvas.draw_line(Point::new(start[0], start[1]-1), Point::new(end[0], end[1]-1));
    canvas.present();
}

pub(crate) fn draw_field(canvas: &mut Canvas<Window>, field : &GolfField){
    for ball in &field.balls{
        graphics::draw_circle(canvas, Point::new(ball[0], ball[1]), 5, Color::RGB(255,0,0));
    }

    for hole in &field.holes{
        graphics::draw_circle(canvas, Point::new(hole[0], hole[1]), 5, Color::RGB(0,0,255));
    }
}
