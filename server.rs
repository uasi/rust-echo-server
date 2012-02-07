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
    let addr = new_sockaddr_in(af_inet, inet_addr(host), port);
    let r = bind_(sock, addr);
    if failure(r) {
        std::io::println(#fmt("cannot bind to %s:%u", host, port as uint));
    }

    std::io::println(#fmt("waiting client on %s:%u...", host, port as uint));
    listen(sock, 1);
    let client_addr = copy addr;
    let client_sock = accept(sock, client_addr);

    std::io::println("connected.");
    let buf = vec::init_elt_mut(1024u, 0u8);
    while true {
        let size = recv(client_sock, buf, 0u);
        if size == 0 { break; }
        else if size < 0 { fail; }
        let s = from_bytes_n(buf, size as uint);
        std::io::print(#fmt("> %s", s));
        send_str(client_sock, s, 0u);
    }

    std::io::println("bye.");
}
