use entity::connection::Connection;
use service::tcp::Tcp;

mod service;
mod entity;

static IP: &str = "localhost";
static PORT: &str = "444";

fn main() {
    let mut tcp = Tcp::new(Connection{ip: &IP, port: &PORT}).unwrap();
}
