use std;

import core::result::*;
import socket::*;

fn from_bytes_n(bytes: [const u8], count: uint) -> str {
    assert count <= vec::len(bytes);
    ret str::from_bytes(vec::slice(bytes, 0u, count));
}

fn main() {
    let host = "0.0.0.0";
    let port = 12345u16;
    let sock = new_tcp_socket();
    let addr = inet_addr(host);
    let sockaddr = new_sockaddr_in(af_inet, addr, port);
    let r = connect(sock, sockaddr);
    if failure(r) {
        fail #fmt("cannot connect to %s:%u", host, port as uint);
    }
    send_str(sock, "HELLO", 0u);
    let buf = vec::init_elt_mut(1024u, 0u8);
    let size = recv(sock, buf, 0u);
    let s = from_bytes_n(buf, size as uint);
    std::io::println(s);
}
