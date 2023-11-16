use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    {
        let socket = UdpSocket::bind(&args[1])?;
        socket.connect(&args[2])?;

        let mut buf = Vec::new();
        buf.resize(args[3].parse::<usize>().unwrap(), 0);

        loop {
            socket.send(&buf)?;
        }
    }
}
