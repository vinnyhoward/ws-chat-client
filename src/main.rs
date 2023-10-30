use socket2::{Domain, Socket, Type};
use std::net::{SocketAddr, TcpListener};

fn main() {
    let socket = Socket::new(Domain::IPV6, Type::STREAM, None).unwrap();
    socket.set_only_v6(false).unwrap();
    
    let address: SocketAddr = "[::1]:12345".parse().unwrap();
    socket.bind(&address.into()).unwrap();
    socket.listen(128).unwrap();

    let listener: TcpListener = socket.into();
}
