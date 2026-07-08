use std::io;
fn main(){
let user_name : String = whoami::username(); // by the way, this is a crate that can be used to get the username of the current user
let mut user_input: String = String::new();
println!("how can i help you, {} today?", user_name);
io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");
println!("if you, {} i am here to help!", user_input.trim());
}