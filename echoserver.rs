fn copy_stream(in : TcpStream, out : TcpStream) {
}

fn handle_client(client_stream : TcpStream) {
    copy_stream(client_stream, client_stream);
}

fn main() {
    println("sup dawg")
}
