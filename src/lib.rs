extern crate zmq;

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ZFrame {
  body: Vec<u8>,
}

impl ZFrame {
  pub fn new(string: &str) -> ZFrame {
    let mut body = Vec::new();
    body.extend_from_slice(string.as_bytes());

    ZFrame { body: body }
  }

  pub fn empty() -> ZFrame {
    ZFrame { body: Vec::new() }
  }
}

impl fmt::Display for ZFrame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let string = String::from_utf8_lossy(&self.body);
    write!(f, "{}", string)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ZMsg {
  frames: Vec<ZFrame>
}

impl ZMsg {
  pub fn new() -> ZMsg {
    ZMsg { frames: Vec::new() }
  }

  // push to the front
  pub fn push(&mut self, frame: ZFrame) {
    self.frames.insert(0, frame);
  }

  pub fn push_string(&mut self, string: &str) {
    self.frames.insert(0, ZFrame::new(string))
  }

  pub fn pop(&mut self) -> ZFrame {
    self.frames.remove(0)
  }

  // push to the back
  pub fn push_back(&mut self, frame: ZFrame) {
    self.frames.push(frame);
  }

  pub fn push_string_back(&mut self, string: &str) {
    self.frames.push(ZFrame::new(string));
  }

  pub fn pop_back(&mut self) -> Option<ZFrame> {
    self.frames.pop()
  }

  pub fn send(&self, socket: &mut zmq::Socket) -> zmq::Result<()> {
    let (last, first) = self.frames.split_last().unwrap();

    for frame in first.iter() {
      try!(socket.send(&frame.body, zmq::SNDMORE));
    }
    try!(socket.send(&last.body, 0));

    Ok(())
  }

  pub fn recv(socket: &mut zmq::Socket) -> zmq::Result<ZMsg> {
    let mut msg = ZMsg::new();

    loop {
      let part = try!(socket.recv_bytes(0));
      msg.push_back(ZFrame { body: part });

      let more_parts = try!(socket.get_rcvmore());
      if !more_parts {
        break;
      }
    }

    Ok(msg)
  }

  pub fn len(&self) -> usize {
    self.frames.len()
  }
}

impl<'a> From<&'a [&'a [u8]]> for ZMsg {
  fn from(slice: &'a [&'a [u8]]) -> ZMsg {
    let mut msg = ZMsg::new();
    for data in slice.iter() {
      let mut body = Vec::new();
      body.extend_from_slice(data);
      msg.push_back(ZFrame { body: body });
    }
    msg
  }
}

impl fmt::Display for ZMsg {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for frame in &self.frames {
      try!(write!(f, "[{}]", frame));
    }

    Ok(())
  }
}
