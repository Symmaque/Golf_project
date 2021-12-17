mod input;
mod types;
use input::field_from_name;
use types::GolfField;
use types::Point;

// Find pivot, i.e. the point with minimal 1st component.
fn find_pivot(field: &GolfField) -> Point {
       // TODO: replace redundant logic
       let ball = field
              .balls
              .iter()
              .reduce(|acc: &Point, cur: &Point| if cur[0] < acc[0] { cur } else { acc })
              .unwrap();
       let hole = field
              .holes
              .iter()
              .reduce(|acc: &Point, cur: &Point| if cur[0] < acc[0] { cur } else { acc })
              .unwrap();
       if ball[0] < hole[0] {
              ball.clone()
       } else {
              hole.clone()
       }
}

/// Score ordering points by angle.
fn order_score([xo, yo]: &Point, [x, y]: &Point) -> f32 {
       (y - yo) / (x - xo + 0.1)
}

/// Order balls and holes according to their angle with pivot.
fn order_field(field: &mut GolfField) {
       let pivot = find_pivot(field);
       field.balls.sort_by(|a, b| {
              order_score(&pivot, a)
                     .partial_cmp(&order_score(&pivot, b))
                     .unwrap()
       });
       field.holes.sort_by(|a, b| {
              order_score(&pivot, a)
                     .partial_cmp(&order_score(&pivot, b))
                     .unwrap()
       });
}

/// Divide a sorted field in half and returns the ball - hole pair dividing it.
fn divide_field(sorted_field: &GolfField) -> ([GolfField; 2], [Point; 2]) {
       // TODO: implement it
       unimplemented!();
}

fn main() {
       let mut field = field_from_name("input1").expect("Cannot load input field.");
       println!("{:?}", &field);
       order_field(&mut field);
       println!("{:?}", &field);
}
