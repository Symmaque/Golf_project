use std::env::current_dir;
use crate::types::{GolfField, Point};
use serde_json::Result;
use std::{fs, io};
use std::fs::File;
use std::io::{BufRead, stdout, Write};

/// Utility function returning inputs path.
pub fn input_path(field_name: &str) -> std::io::Result<std::path::PathBuf> {
    Ok(fs::canonicalize(format!(
        "{}{}{}.{}",
        current_dir().unwrap().to_str().unwrap(), "\\inputs\\", field_name, "txt"
    ))?)
}

/// Read and parse inputs golf field data (balls & holes) from path.
/// ### Example
/// ```
/// let field: GolfField = field_from_path(&input_path("input1"));
/// ```
pub fn field_from_path(path: &std::path::PathBuf) -> Result<GolfField> {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();

    let mut balls : Vec<Point> = Vec::new();
    let mut holes : Vec<Point> = Vec::new();

    let mut reference = &mut balls;

    for (i, line) in lines.enumerate() {
        let line_ref = line.unwrap();
        if i == 0 && !line_ref.eq("balls") {
            println!("balls are missing at the first line");
        }
        if line_ref.eq("balls"){
            continue;
        }
        if line_ref.eq("holes") {
            reference = &mut holes;
            continue;
        }
        let string = line_ref.replace("\n","");
        let coords : Vec<&str> = string.split(" ").collect();

        reference.push([coords[0].parse::<i32>().unwrap(), coords[1].parse::<i32>().unwrap()]);
    }
    let mut field: GolfField = GolfField{balls : Vec::new(), holes : Vec::new()};

    for ball in balls{
        field.balls.push([ball[0], ball[1]]);
    }
    for hole in holes{
        field.holes.push([hole[0], hole[1]]);
    }
    // make sure that there are as many balls as holes on the field
    assert_eq!(
        field.balls.len(),
        field.holes.len(),
        "There are not as many balls as there are holes on the field."
    );
    Ok(field)
}

/// Read and parse inputs golf field data (balls & holes) from name.
/// ### Example
/// ```
/// let field: GolfField = field_from_name("input1").expect("Cannot load field.");
/// ```
pub fn field_from_name(field_name: &str) -> Result<GolfField> {
    // retrieve inputs path
    let path = input_path(field_name).expect("Input path doesn\'t exist.");
    // read and parse inputs field
    let field = field_from_path(&path).expect("Cannot read and parse inputs field.");
    Ok(field)
}

#[test]
fn should_correctly_load_input() {
    let field = field_from_name("input1").expect("Cannot load inputs field.");
    assert_eq!(
        format!("{:?}", field),
        "GolfField { holes: [[400, 300], [120, 220], [380, 320], [580, 180]], balls: [[100, 100], [280, 280], [320, 380], [480, 220]] }"
    )
}

///
/// Get user's input file
///
pub fn get_input_file () -> String{
    print!("Please enter the name of the input file (without the extension .txt) :");
    let _ = stdout().flush();
    let mut name_file = String::new();
    io::stdin().read_line(&mut name_file).expect("read error");
    name_file.replace("\n", "").replace("\r", "")
}