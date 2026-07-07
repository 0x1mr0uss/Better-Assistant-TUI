use std::io;
fn main(){
let mut user_input: String = String::new();
io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");
println!("{}", user_input.trim());
}