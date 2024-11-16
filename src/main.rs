use entity::connection::Connection;
use service::{enumerator::Enumerator, tcp::Tcp};

mod service;
mod entity;
mod test;

static IP: &str = "localhost";
static PORT: &str = "444";

fn main() {
    let mut tcp = Tcp::new(Connection{ip: &IP, port: &PORT}).unwrap();
    let enumerator = Enumerator::new(&tcp);
}
