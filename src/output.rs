use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use crate::types;


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