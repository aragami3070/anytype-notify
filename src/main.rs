use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Ip(pub String);

fn main() {
    dotenv().ok();

    let anytype_ip = Ip(std::env::var("ANYTYPE_IP").expect("ANYTYPE_IP must be set in .env."));

    println!("anytype_ip: {}", anytype_ip.0);
}
