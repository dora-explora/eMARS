use std::env::args;

mod sim;

fn main() {
    let args: Vec<String> = args().collect();
    sim::init("warriors/imp.red", "warriors/test.red", 50)
}