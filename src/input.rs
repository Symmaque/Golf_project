use std::env::current_dir;
use crate::types::GolfField;
use serde_json::Result;
use std::fs;

/// Utility function returning inputs path.
pub fn input_path(field_name: &str) -> std::io::Result<std::path::PathBuf> {
    Ok(fs::canonicalize(format!(
        "{}{}{}.{}",
        current_dir().unwrap().to_str().unwrap(), "\\inputs\\", field_name, "json"
    ))?)
}

/// Read and parse inputs golf field data (balls & holes) from path.
/// ### Example
/// ```
/// let field: GolfField = field_from_path(&input_path("input1"));
/// ```
pub fn field_from_path(path: &std::path::PathBuf) -> Result<GolfField> {
    // read some JSON inputs data as a String. Maybe this comes from the user.
    let raw_data = fs::read_to_string(path).expect("Unable to read file.");
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