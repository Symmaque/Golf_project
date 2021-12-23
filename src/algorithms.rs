use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::graphics::draw_segment;
use crate::types::{GolfField, Point};

pub(crate) fn distance (m : Point, p : Point) -> i32{
    let diff_x = m[0] - p[0];
    let diff_y = m[1] - p[1];
    diff_x*diff_x + diff_y*diff_y
}

pub(crate) fn golf(canvas: &mut Canvas<Window>, G : GolfField){
    let B = &G.balls;
    let H = &G.holes;

    if B.len() <= 2 {
        conquer(canvas, B, H);
    } else {
        divide(canvas, B, H);
    }
}

pub(crate) fn conquer(canvas: &mut Canvas<Window>, B : &Vec<Point>, H : &Vec<Point>){
    if B.len() == 1 {
        draw_segment(canvas, B[0], H[0], Color::RGB(0,0,0));
    } else if distance(B[0], H[0]) + distance(B[1], H[1]) < distance(B[0], H[1]) + distance(B[1], H[0]) {
        draw_segment(canvas, B[0], H[0], Color::RGB(0,0,0));
        draw_segment(canvas, B[1], H[1], Color::RGB(0,0,0));
    }else {
        draw_segment(canvas, B[0], H[1], Color::RGB(0,0,0));
        draw_segment(canvas, B[1], H[0], Color::RGB(0,0,0));
    }
}

/// O(n) to find origin
pub(crate) fn find_origin<'a>(B : &'a Vec<Point>, H : &'a Vec<Point>) -> (&'a Point, bool){
    let mut origin : &Point = &B[0];
    let mut origin_type :bool = true;    // true for ball
    for ball in B {
        if ball[0] < origin[0] {
            origin = ball;
        }
    }
    for hole in H {
        if hole[0] < origin[0] {
            origin = hole;
            origin_type = false;    // false for hole
        }
    }
    (origin, origin_type)
}



pub(crate) fn angle (M: &Point, N: &Point) -> f32{
    if M[0] == N[0] && M[1] > N[1]{
        -90f32
    } else if M[0] == N[0] && M[1] < N[1] {
        90f32
    } else {
        let diff_y = (N[1] - M[1]) as f32;
        let diff_x = (N[0] - M[0]) as f32;
        let quotient: f32 = diff_y / diff_x;
        quotient.tan()
    }
}

pub(crate) fn compute_angles<'a> (B : &'a Vec<Point>, H: &'a Vec<Point>, origin : &'a Point, origin_type: bool) -> (Vec<(f32, &'a Point)>, Vec<(f32, &'a Point)>){
    let mut alpha_balls : Vec<(f32, &Point)> = Vec::new();
    let mut alpha_holes: Vec<(f32, &Point)> = Vec::new();

    for ball in B{
        if origin_type && ball[0] == origin[0] && ball[1] == origin[1] {
            continue;
        }
        alpha_balls.push((angle(origin, ball), ball));
    }

    for hole in H{
        if !origin_type && hole[0] == origin[0] && hole[1] == origin[1] {
            continue;
        }
        alpha_holes.push((angle(origin, hole), hole));
    }

    //TODO : sort vectors by angle
    alpha_holes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    alpha_balls.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());


    println!("Holes");
    for angle in &alpha_holes{
        println!("angle = {:?} for the point {:?} , {:?}", angle.0, angle.1[0], angle.1[1]);
    }

    println!("Balls");
    for angle in &alpha_balls{
        println!("angle = {:?} for the point {:?} , {:?}", angle.0, angle.1[0], angle.1[1]);
    }

    (alpha_balls, alpha_holes)
}

pub(crate) fn divide(canvas: &mut Canvas<Window>, B : &Vec<Point>, H : &Vec<Point>){
    let (origin, origin_type) = find_origin(B, H);
    println!("x_0 = {:?} and y_0 = {:?}", origin[0], origin[1]);
    println!("type = {:?}", origin_type);
    let (alpha_balls, alpha_holes) = compute_angles(B,H,origin,origin_type);

    if origin_type {
        if alpha_holes[0].0 < alpha_balls[0].0{
            draw_segment(canvas, *origin, *alpha_holes[0].1, Color::RGB(255,255,255))
            //TODO : golf with origin and alpha_holes[0].0 removed
            //golf(canvas, )
        } else if alpha_holes[alpha_holes.len()-1].0 > alpha_balls[alpha_balls.len()-1].0 {
            draw_segment(canvas, *origin, *alpha_holes[alpha_holes.len()-1].1, Color::RGB(255,255,255))
            //TODO : golf with origin and alpha_holes[alpha_holes.len()-1].0 removed
        } else {
            for i in 2..(n-1) {
                if alpha_holes[i].0 > alpha_balls[i-1].0 && alpha_holes[i].0 < alpha_balls[i].0 {
                    draw_segment(canvas, *origin, *alpha_holes[i].1);
                    // TODO : slice vectors into left and right parts
                    // TODO : run golf on the left and right parts
                }
            }
        }



    } else {


    }
}