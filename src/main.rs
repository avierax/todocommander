mod model;
mod config;

fn main() {
    config::parse_arguments(&mut std::env::args());
    println!("Hello, world! ");
}
