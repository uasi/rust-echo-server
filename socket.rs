use std;

// enum
export addr_family;
export socket_type;
export protocol_type;

// fn
export accept;
export bind_;
export close;
export connect;
export inet_addr;
export listen;
export new_sockaddr_in;
export new_socket;
export new_tcp_socket;
export recv;
export send;
export send_str;

import core::ctypes::*;
import core::result::{ok, err};

// FIXME: fill missing enums

#[cfg(target_os = "macos")]
enum addr_family {
    af_inet = 2,
    af_inet6 = 26,
}

#[cfg(target_os = "linux")]
enum addr_family {
    af_inet = 2,
    af_inet6 = 10,
}

enum socket_type {
    sock_stream = 1,
    sock_dgram = 2,
}

enum protocol_type {
    ipproto_ip = 0,
}

type in_addr = {
    s_addr: u32,
};

type sockaddr_in = {
    mutable sin_len: u8,
    mutable sin_family: i8,
    mutable sin_port: u16,
    mutable sin_addr: in_addr,
    mutable sin_zero: u64,
};

#[cfg(target_os = "linux")] // untested
#[cfg(target_os = "macos")]
#[nolink]
#[abi = "cdecl"]
native mod libc {
    fn accept(socket: c_int, address: *mutable sockaddr_in, address_len: *mutable uint32_t) -> c_int;
    fn bind(socket: c_int, address: *sockaddr_in, address_len: uint32_t) -> c_int;
    fn close(socket: c_int);
    fn connect(socket: c_int, address: *sockaddr_in, address_len: uint32_t) -> c_int;
    fn socket(af: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn htons(host: u16) -> u16;
    fn inet_addr(cp: *u8) -> u32;
    fn listen(socket: c_int, backlog: c_int) -> c_int;
    fn recv(socket: c_int, buffer: uintptr_t, length: size_t, flags: c_int) -> ssize_t;
    fn send(socket: c_int, buffer: uintptr_t, length: size_t, flags: c_int) -> ssize_t;
}

resource socket(s: c_int) {
    libc::close(s);
}

fn new_socket(af: addr_family, st: socket_type, pt: protocol_type) -> socket {
    ret _new_socket(af, st, pt);

    #[cfg(target_os = "linux")] // untested
    #[cfg(target_os = "macos")]
    fn _new_socket(af: addr_family, st: socket_type, pt: protocol_type) -> socket {
        let s = libc::socket(af as c_int, st as c_int, pt as c_int);
        ret socket(s);
    }
}

fn new_tcp_socket() -> socket {
    ret new_socket(af_inet, sock_stream, ipproto_ip);
}

fn new_udp_socket() -> socket {
    ret new_socket(af_inet, sock_dgram, ipproto_ip);
}

fn new_sockaddr_in(af: addr_family, addr: u32, port: u16) -> sockaddr_in {
    ret _new_sockaddr_in(af, addr, port);

    fn _new_sockaddr_in(af: addr_family, addr: u32, port: u16) -> sockaddr_in {
        ret {
            mutable sin_len: 16 as u8,
            mutable sin_family: af as i8,
            mutable sin_port: libc::htons(port),
            mutable sin_addr: {s_addr: addr},
            mutable sin_zero: 0u64,
        };
    }
}


fn connect(s: socket, a: sockaddr_in) -> result::t<int, int> {
    let c_sock = *s;
    let r = libc::connect(c_sock, ptr::addr_of(a), 16 as uint32_t) as int;
    ret alt r { 0 { ok(r) } _ { err(r) } };
}

fn bind_(s: socket, a: sockaddr_in) -> result::t<int, int> {
    let c_sock = *s;
    let r = libc::bind(c_sock, ptr::addr_of(a), 16 as uint32_t) as int;
    ret alt r { 0 { ok(r) } _ { err(r) } };
}

fn listen(s: socket, backlog: int) -> int {
    ret libc::listen(*s, backlog as c_int) as int;
}

fn accept(s: socket, &a: sockaddr_in) -> socket {
    let addr_len = 0 as uint32_t;
    let c_sock = libc::accept(*s, ptr::mut_addr_of(a), ptr::mut_addr_of(addr_len));
    ret socket(c_sock);
}

// FIXME: rename to inet_aton() ?
fn inet_addr(str_addr: str) -> u32 {
    ret str::as_buf(str_addr) {|a| libc::inet_addr(a) };
}

// FIXME: take size
fn send(s: socket, buf: [const u8], flags: uint) -> int unsafe {
    let sz = libc::send(*s,
                        vec::unsafe::to_ptr(buf) as uintptr_t,
                        vec::len(buf),
                        flags as c_int);
    ret sz as int;
}

fn send_str(s: socket, buf: str, flags: uint) -> int unsafe {
    let saddr = ptr::addr_of(buf);
    let vaddr: *[u8] = ::unsafe::reinterpret_cast(saddr);
    ret send(s, *vaddr, flags);
}

fn recv(s: socket, &buf: [mutable u8], flags: uint) -> int unsafe {
    let sz = libc::recv(*s,
                        vec::unsafe::to_ptr(buf) as uintptr_t,
                        vec::len(buf),
                        flags as c_int);
    ret sz as int;
}

// FIXME: This may not be used
fn close(s: socket) {
    libc::close(*s);
}
