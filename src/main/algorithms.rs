use std::f64::consts::PI;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::graphics::draw_segment;
use crate::output::save_segment;
use crate::types::{Point};

///
/// Function computing the squared distance between two points
///
pub(crate) fn distance(m: &Point, p: &Point) -> f64 {
    let diff_x = m[0] - p[0];
    let diff_y = m[1] - p[1];
    f64::sqrt((diff_x * diff_x + diff_y * diff_y) as f64)
}
///
/// main algorithm
///
pub(crate) fn golf(canvas: &mut Canvas<Window>, b: &mut Vec<Point>, h: &mut Vec<Point>, filename: &String) {
    if b.len() <= 2 {
        conquer(canvas, b, h, filename);
    } else {
        divide(canvas, b, h, filename);
    }
}
///
/// Conquer algorithm
///
pub(crate) fn conquer(canvas: &mut Canvas<Window>, b: &Vec<Point>, h: &Vec<Point>, filename: &String) {
    if b.len() == 0 {
        ()
    }
    if b.len() == 1 {
        draw_segment(canvas, &b[0], &h[0], Color::RGB(0, 0, 0));
        save_segment(&b[0], &h[0], filename).unwrap();
    } else if distance(&b[0], &h[0]) + distance(&b[1], &h[1]) < distance(&b[0], &h[1]) + distance(&b[1], &h[0]) {   //if len = 2, apply criteria of minimum distance
        draw_segment(canvas, &b[0], &h[0], Color::RGB(0, 0, 0));
        draw_segment(canvas, &b[1], &h[1], Color::RGB(0, 0, 0));
        save_segment(&b[0], &h[0], filename).unwrap();
        save_segment(&b[1], &h[1], filename).unwrap();
    } else {
        draw_segment(canvas, &b[0], &h[1], Color::RGB(0, 0, 0));
        draw_segment(canvas, &b[1], &h[0], Color::RGB(0, 0, 0));
        save_segment(&b[0], &h[1], filename).unwrap();
        save_segment(&b[1], &h[0], filename).unwrap();
    }
}

///
/// Function finding the origin
/// O(n) to find origin
pub(crate) fn find_origin(b: &Vec<Point>, h: &Vec<Point>) -> (Point, bool) {
    let mut origin: Point = [b[0][0], b[0][1]];
    let mut origin_type: bool = true;    // true for ball
    for &ball in b {
        if ball[0] < origin[0] {
            origin[0] = ball[0];
            origin[1] = ball[1];
        }
    }
    for &hole in h {
        if hole[0] < origin[0] {
            origin[0] = hole[0];
            origin[1] = hole[1];
            origin_type = false;    // false for hole
        }
    }
    (origin, origin_type)
}
///
/// Function computing the value of the angle between 2 points
///
pub(crate) fn angle(m: &Point, n: &Point) -> f32 {
    if m[0] == n[0] && m[1] > n[1] {
        -(PI as f32 / 2 as f32)
    } else if m[0] == n[0] && m[1] < n[1] {
        PI as f32 / 2 as f32
    } else {
        let diff_y = (n[1] - m[1]) as f32;
        let diff_x = (n[0] - m[0]) as f32;
        let quotient: f32 = diff_y / diff_x;
        quotient.atan()
    }
}

///
/// Function computing the angles of all the points comparing to the origin
/// O(nlogn) time complexity
///
pub(crate) fn compute_angles(b: &Vec<Point>, h: &Vec<Point>, origin: &Point, origin_type: bool) -> (Vec<(f32, Point)>, Vec<(f32, Point)>) {
    let mut beta: Vec<(f32, Point)> = Vec::new();  // same type as the origin (n-1)
    let mut alpha: Vec<(f32, Point)> = Vec::new(); // other type as the origin (n)

    // O(n)
    if origin_type {
        for ball in b {
            if ball[0] == origin[0] && ball[1] == origin[1] {
                continue;   // same point
            }
            beta.push((angle(origin, ball), [ball[0], ball[1]]));
        }

        for hole in h {
            alpha.push((angle(origin, hole), [hole[0], hole[1]]));
        }
    } else {
        for hole in h {
            if hole[0] == origin[0] && hole[1] == origin[1] {
                continue;
            }
            beta.push((angle(origin, hole), [hole[0], hole[1]]));
        }
        for ball in b {
            alpha.push((angle(origin, ball), [ball[0], ball[1]]));
        }
    }

    //Sort vectors by angle ascending O(nlogn)
    alpha.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    beta.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    (alpha, beta)
}


///
/// Divide part of the algorithm
///
pub(crate) fn divide(canvas: &mut Canvas<Window>, b: &mut Vec<Point>, h: &mut Vec<Point>, filename: &String) {
    let (origin, origin_type) = find_origin(b, h);  // O(n)
    let (alpha, beta) = compute_angles(&b, &h, &origin, origin_type); // O(nlogn)
    let n = alpha.len();

    // if all the other points are in one part
    if alpha[0].0 < beta[0].0 {
        draw_segment(canvas, &origin, &alpha[0].1, Color::RGB(0, 0, 0));
        save_segment(&origin, &alpha[0].1, filename).unwrap();
        if origin_type {
            b.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            h.retain(|&x| (x[0] != (alpha[0].1)[0] || x[1] != (alpha[0].1)[1]));
        } else {
            h.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            b.retain(|&x| (x[0] != alpha[0].1[0] || x[1] != alpha[0].1[1]));
        }
        golf(canvas, b, h, filename);
    } else if alpha[n - 1].0 > beta[n - 2].0 { //if all the other points are in the other part
        draw_segment(canvas, &origin, &alpha[n - 1].1, Color::RGB(0, 0, 0));
        save_segment(&origin, &alpha[n - 1].1, filename).unwrap();
        if origin_type {
            b.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            h.retain(|&x| (x[0] != alpha[n - 1].1[0] || x[1] != alpha[n - 1].1[1]));
        } else {
            h.retain(|&x| (x[0] != origin[0] || x[1] != origin[1]));
            b.retain(|&x| (x[0] != alpha[n - 1].1[0] || x[1] != alpha[n - 1].1[1]));
        }
        golf(canvas, b, h, filename);
    } else {   // if there are points in the two parts : O(n)
        for i in 1..(n - 1) {
            if alpha[i].0 > beta[i - 1].0 && alpha[i].0 < beta[i].0 {
                draw_segment(canvas, &origin, &alpha[i].1, Color::RGB(0, 0, 0));
                save_segment(&origin, &alpha[i].1, filename).unwrap();
                let mut b_left: Vec<Point> = Vec::new();
                let mut h_left: Vec<Point> = Vec::new();
                let mut b_right: Vec<Point> = Vec::new();
                let mut h_right: Vec<Point> = Vec::new();

                if origin_type {
                    for k in 0..i {
                        b_left.push(beta[k].1);
                        h_left.push(alpha[k].1);
                    }
                    for k in i..(n - 1) {
                        b_right.push(beta[k].1);
                        h_right.push(alpha[k + 1].1);
                    }
                } else {
                    for k in 0..i {
                        b_left.push(alpha[k].1);
                        h_left.push(beta[k].1);
                    }
                    for k in i..(n - 1) {
                        b_right.push(alpha[k + 1].1);
                        h_right.push(beta[k].1);
                    }
                }
                golf(canvas, &mut b_left, &mut h_left, filename);   // run on a part
                golf(canvas, &mut b_right, &mut h_right, filename); // run on the other part
                return;
            }
        }
    }
}