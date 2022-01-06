use std::env::current_dir;
use std::{fs, io};
use std::fs::File;
use std::io::{stdout, Write};
use crate::types;


///
/// Function saving the coordinates of segments in an output file
///
pub(crate) fn save_segment(start: &types::Point, end: &types::Point, filename: &String) -> std::io::Result<()> {
    let mut output: File = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!(
            "{}{}{}",
            current_dir().unwrap().to_str().unwrap(), "\\output\\", filename
        ))
        .unwrap();
    writeln!(output, "{}", format!("[{}, {}] to [{}, {}]", start[0], start[1], end[0], end[1]))
}
///
/// Get user's output file
///
pub fn get_output_file() -> String{
    print!("Please enter the name of the output file (without the extension .txt) :");
    let _ = stdout().flush();
    let mut name_file = String::new();
    io::stdin().read_line(&mut name_file).expect("read error");
    name_file.replace("\n", "").replace("\r", "")
}