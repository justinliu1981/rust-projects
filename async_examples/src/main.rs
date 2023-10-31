use socket2::Socket;


fn main() {
    println!("Hello, world!");
}
#[allow(dead_code)]
pub struct SocketRead <'a> {
    socket: &'a Socket,
}
