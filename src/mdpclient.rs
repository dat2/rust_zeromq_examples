extern crate zmq;
extern crate zhelpers;

use zhelpers::*;

fn make_mdp_request<'a>(service: &'a str, mut body: ZMsg) -> ZMsg {
  body.push_string(service);
  body.push_string("MDPC01");
  body
}

// error message
fn unwrap_reply(service: &str, mut reply: ZMsg) -> Result<ZMsg, &str> {
  if reply.len() < 3 {
    Err("Invalid reply: not enough frames.")
  } else {
    let protocol_header = reply.pop();
    let reply_service = reply.pop();

    if protocol_header != ZFrame::new("MDPC01") {
      Err("Invalid reply: invalid protocol header.")
    } else if reply_service != ZFrame::new(service) {
      Err("Invalid reply: invalid service responded.")
    } else {
      Ok(reply)
    }
  }
}

fn main() {

  let mut context = zmq::Context::new();
  let mut socket = context.socket(zmq::REQ).unwrap();
  socket.connect("tcp://127.0.0.1:5555").unwrap();

  loop {
    let mut body = ZMsg::new();
    body.push_string("Hello");
    body = make_mdp_request("COFFEE", body);
    body.send(&mut socket);

    let reply = unwrap_reply("COFFEE", ZMsg::recv(&mut socket).unwrap());
    println!("Reply: {}", reply.unwrap());
  }
}
