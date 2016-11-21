extern crate zmq;
extern crate zhelpers;

use zhelpers::*;

fn make_mdp_reply<'a>(service: &'a str, mut body: ZMsg) -> ZMsg {
  body.push_string(service);
  body.push_string("MDPC01");
  body
}

fn main() {
  let mut context = zmq::Context::new();
  let mut socket = context.socket(zmq::ROUTER).unwrap();
  socket.bind("tcp://127.0.0.1:5555").unwrap();

  loop {
    let mut request = ZMsg::recv(&mut socket).unwrap();
    println!("Request: {}", request);

    let id = request.pop();
    let empty = request.pop();

    let mut reply = ZMsg::new();
    reply.push_string("World");
    reply = make_mdp_reply("COFFEE", reply);
    reply.push(ZFrame::empty());
    reply.push(id);
    reply.send(&mut socket);
  }
}
