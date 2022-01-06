#![allow(non_snake_case)]

use std::fs::File;
use std::{fs, io};
use std::io::{stdout, Write};
use std::env::current_dir;
use rand::Rng;


extern crate rand;


fn main() {
    //get the path of the file inputs.txt
    let path = format!(
        "{}{}{}",
        current_dir().unwrap().to_str().unwrap(), "\\inputs\\", "inputs.txt"
    );
    // get the number of balls / holes
    let n = get_n();

    // check if the file inputs.txt exists
    match fs::remove_file(&path) {
        Ok(_response) => println!("File {:?} already existed so it has been erased before use", &path),
        Err(_error) => ()
    };

    //open file inputs.txt
    let mut file: File = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    //write balls' coordinates
    write_points(&mut file, &String::from("balls"), n);
    //write holes' coordinates
    write_points(&mut file, &String::from("holes"), n);
}

///
/// Function writing the random coordinates of n points so that that we can parse them
///
fn write_points(file: &mut File, name: &String, n: i32) {
    //random generator
    let mut rng = rand::thread_rng();

    //max of coordinates
    let max_x = 800;
    let max_y = 600;

    write!(file, "{}\n", name).unwrap();
    let mut x: i32;
    let mut y: i32;
    for _i in 0..n - 1 {
        x = rng.gen_range(0, max_x);
        y = rng.gen_range(0, max_y);
        write!(file, "{}", format!("{} {}\n", x, y)).unwrap();
    }
    x = rng.gen_range(0, max_x);
    y = rng.gen_range(0, max_y);
    write!(file, "{}", format!("{} {}\n", x, y)).unwrap();
}

///
/// Function getting the user's number of balls/holes
///
fn get_n() -> i32 {
    print!("Please enter n and press Enter : ");
    let _ = stdout().flush();
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");
    a_str = (&a_str).replace("\n", "");
    a_str = (&a_str).replace("\r", "");
    let result = a_str.parse::<i32>();
    let result = match result {
        Ok(result) => result,
        Err(error) => {
            println!("{}", error);
            0
        }
    };
    result
}

