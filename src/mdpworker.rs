extern crate titanic;

use titanic::zmq;
// use titanic::zmq::{SocketType};

fn main() {
  let mut ctx = zmq::ZContext::new();
  // let socket = ctx.socket(SocketType::Rep);

  println!("Hello World");
}
