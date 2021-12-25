use std::f64::consts::PI;
use sdl2::pixels::Color;
use sdl2::rect::{max_int_value, min_int_value};
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::graphics::draw_segment;
use crate::types::{GolfField, Point};

pub(crate) fn distance (m : &Point, p : &Point) -> i32{
    let diff_x = m[0] - p[0];
    let diff_y = m[1] - p[1];
    diff_x*diff_x + diff_y*diff_y
}

pub(crate) fn golf(canvas: &mut Canvas<Window>, B : &mut Vec<Point>, H : &mut Vec<Point>){
    if B.len() <= 2 {
        conquer(canvas, B, H);
    } else {
        divide(canvas, B, H);
    }
}

pub(crate) fn conquer(canvas: &mut Canvas<Window>, B : &Vec<Point>, H : &Vec<Point>){
    if B.len() == 0 {
        ()
    }
    if B.len() == 1 {
        draw_segment(canvas, &B[0], &H[0], Color::RGB(0,0,0));
    } else if distance(&B[0], &H[0]) + distance(&B[1], &H[1]) < distance(&B[0], &H[1]) + distance(&B[1], &H[0]) {
        draw_segment(canvas, &B[0], &H[0], Color::RGB(0,0,0));
        draw_segment(canvas, &B[1], &H[1], Color::RGB(0,0,0));
    }else {
        draw_segment(canvas, &B[0], &H[1], Color::RGB(0,0,0));
        draw_segment(canvas, &B[1], &H[0], Color::RGB(0,0,0));
    }
}

/// O(n) to find origin
pub(crate) fn find_origin(B : &Vec<Point>, H : &Vec<Point>) -> (Point, bool){
    let mut origin : Point = [B[0][0],B[0][1]];
    let mut origin_type :bool = true;    // true for ball
    for &ball in B {
        if ball[0] < origin[0] {
            origin[0] = ball[0];
            origin[1] = ball[1];
        }
    }
    for &hole in H {
        if hole[0] < origin[0] {
            origin[0] = hole[0];
            origin[1] = hole[1];
            origin_type = false;    // false for hole
        }
    }
    (origin, origin_type)
}

pub(crate) fn angle (M: &Point, N: &Point) -> f32{
    if M[0] == N[0] && M[1] > N[1]{
        -(PI as f32/2 as f32)
    } else if M[0] == N[0] && M[1] < N[1] {
        (PI as f32/2 as f32)
    } else {
        let diff_y = (N[1] - M[1]) as f32;
        let diff_x = (N[0] - M[0]) as f32;
        let quotient: f32 = diff_y / diff_x;
        quotient.atan()
    }
}

//compute angles implements the choice



pub(crate) fn compute_angles (B : &Vec<Point>, H: &Vec<Point>, origin : &Point, origin_type: bool) -> (Vec<(f32, Point)>, Vec<(f32, Point)>){
    let mut beta: Vec<(f32, Point)> = Vec::new();  // same type as the origin (n-1)
    let mut alpha: Vec<(f32, Point)> = Vec::new(); // other type as the origin (n)

    if origin_type{
        for ball in B{
            if ball[0] == origin[0] && ball[1] == origin[1] {
                continue;   // same point
            }
            beta.push((angle(origin, ball), [ball[0], ball[1]]));
        }

        for hole in H{
            alpha.push((angle(origin, hole), [hole[0], hole[1]]));
        }
    } else {
        for hole in H{
            if hole[0] == origin[0] && hole[1] == origin[1] {
                continue;
            }
            beta.push((angle(origin, hole), [hole[0], hole[1]]));
        }
        for ball in B{
            alpha.push((angle(origin, ball), [ball[0], ball[1]]));
        }
    }

    //TODO : sort vectors by angle
    alpha.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    beta.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    /*
    println!("Holes");
    for angle in &alpha {
        println!("angle = {:?} for the point {:?} , {:?}", angle.0, angle.1[0], angle.1[1]);
    }

    println!("Balls");
    for angle in &beta {
        println!("angle = {:?} for the point {:?} , {:?}", angle.0, angle.1[0], angle.1[1]);
    }
    */
    (alpha, beta)
}

pub(crate) fn divide(canvas: &mut Canvas<Window>, B: &mut Vec<Point>, H : &mut Vec<Point>){
    let (mut origin, origin_type) = find_origin(B, H);
    let (alpha, beta) = compute_angles(&B, &H, &origin, origin_type);
    let n = alpha.len();

    if alpha[0].0 < beta[0].0{
        draw_segment(canvas, &origin, &alpha[0].1, Color::RGB(0, 0, 0));
        if origin_type {
            B.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            H.retain(|&x| (x[0] != (alpha[0].1)[0] || x[1] != (alpha[0].1)[1]));
        } else {
            H.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            B.retain(|&x| (x[0] != alpha[0].1[0] || x[1] != alpha[0].1[1]));
        }
        golf(canvas, B, H);
    } else if alpha[n-1].0 > beta[n-2].0 {
        println!("alpha [n-1] = [{:?},{:?}]", alpha[n-1].1[0], alpha[n-1].1[1]);
        draw_segment(canvas, &origin, &alpha[n-1].1, Color::RGB(0, 0, 0));
        if origin_type {
            B.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            H.retain(|&x| (x[0] != alpha[n-1].1[0] || x[1] != alpha[n-1].1[1]));
        } else {
            H.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            B.retain(|&x| (x[0] != alpha[n-1].1[0] || x[1] != alpha[n-1].1[1]));
        }
        golf(canvas, B, H);
    } else {
        let mut bool : bool = false;
        for i in 1..(n-1) {
            if alpha[i].0 > beta[i-1].0 && alpha[i].0 < beta[i].0 {
                draw_segment(canvas, &origin, &alpha[i].1, Color::RGB(0, 0, 0));
                let mut B_left: Vec<Point> = Vec::new();
                let mut H_left: Vec<Point>= Vec::new();
                let mut B_right: Vec<Point> = Vec::new();
                let mut H_right: Vec<Point> = Vec::new();

                if origin_type {
                    for k in 0..i {
                        B_left.push(beta[k].1);
                        H_left.push(alpha[k].1);
                    }
                    for k in i..(n-1){
                        B_right.push(beta[k].1);
                        H_right.push(alpha[k+1].1);
                    }
                } else {
                    for k in 0..i {
                        B_left.push(alpha[k].1);
                        H_left.push(beta[k].1);
                    }
                    for k in i..(n-1){
                        B_right.push(alpha[k].1);
                        H_right.push(beta[k+1].1);
                    }
                }
                bool = true;
                golf(canvas, &mut B_left, &mut H_left);
                golf(canvas, &mut B_right, &mut H_right);
                return;
            }
            if i == (n-2) && !bool {
                println!("Should not print this");
            }
        }
    }
}