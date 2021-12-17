mod input;
mod types;
use input::field_from_name;

fn main() {
       let field = field_from_name("input1").expect("Cannot load input field.");
       println!("{:?}", field);
}
