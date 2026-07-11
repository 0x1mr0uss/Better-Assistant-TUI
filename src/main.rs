mod about;
use std::io;
use local_ip_address::local_ip;
fn main(){
let user_name : String = whoami::username();  
let mut user_input: String = String::new();

println!("how can i help you, {} today?", user_name);
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
let trimmed_input = user_input.trim().to_lowercase();

if trimmed_input == "ip" {
    let user_ip = local_ip().unwrap_or_else(|_| std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)));
    println!(" your ip is {user_ip} sir {user_name}");
}else if trimmed_input == "about" {
    about::about_better(&user_name);
}else {
    println!("sorry, {user_name} that's Out of my service.")
}
}