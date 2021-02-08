use std::{
  error,
  io::{self, prelude::*},
  net, str, thread,
};

fn main() -> Result<(), Box<dyn error::Error>>{
  // (1)ソケットの生成とローカルアドレスへの紐付け
  let listener = net::TcpListener::bind("127.0.0.1:5000")?;
  loop {
    let (stream, _) = listener.accept()?; //(2)接続の待受
    thread::spawn(move || {
      handler(stream).unwrap();
    });
  }
}

// クライアントが接続しにきたときの処理
fn handler(mut stream: net::TcpStream) -> Result<(), Box<dyn error::Error>> {
  println!("incoming connection from {}", stream.peer_addr()?);
  loop {
    let mut reader = io::BufReader::new(&stream);
    let mut buf = vec![];
    match reader.read_until(b'\n', &mut buf)? { // (3)ソケットから読み出し
      0 => {
        println!("connect closed");
        return Ok(())
      },
      n => {
        print!("{}", str::from_utf8(&buf[..n])?);
        stream.write_all(&buf[..n])?;
      }

    }
  }
}