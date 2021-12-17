use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

/// Spatial representation of a ball or hole.
type Point = [f32; 2];

/// Golf field, set of balls and holes.
#[derive(Debug, Serialize, Deserialize)]
struct GolfField {
       holes: Vec<Point>,
       balls: Vec<Point>,
}

/// Utility function returning input path.
fn input_path(input_name: &str) -> std::io::Result<std::path::PathBuf> {
       Ok(fs::canonicalize(format!(
              "{}{}.{}",
              "../inputs/", input_name, "json"
       ))?)
}

/// Read and parse input golf field data (balls & holes).
/// ### Example
/// ```
/// let field: GolfField = input_field(&input_path("input1"));
/// ```
fn input_field(path: &std::path::PathBuf) -> Result<GolfField> {
       // read some JSON input data as a String. Maybe this comes from the user.
       let raw_data = fs::read_to_string(path).expect("Unable to read file");
       // parse the string of data into a GolfField object
       let field: GolfField = serde_json::from_str(&raw_data.to_string())?;
       // make sure that there are as many balls as holes on the field
       assert_eq!(
              field.balls.len(),
              field.holes.len(),
              "There are not as many balls as there are holes on the field."
       );
       Ok(field)
}

fn main() {
       // retreive input path
       let path = input_path("input1").expect("Input path doesn\'t exist.");
       // read and parse input data
       let field = input_field(&path).expect("Cannot read input data.");
       println!("{:?}", field);
}

#[test]
fn should_fail() {
       unimplemented!()
}
