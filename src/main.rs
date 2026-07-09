use std::io;
use local_ip_address::local_ip;
fn main(){
let user_name : String = whoami::username(); // by the way, this is a crate that can be used to get the username of the current user
let mut user_ip = local_ip().unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
let mut user_input: String = String::new();
println!("how can i help you, {} today?", user_name);
io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to read line");
if user_input.trim() == "Ip" {
    println!(" your ip is {user_ip} sir {user_name}");
};
}