
use std::convert::TryInto;
use std::str::FromStr;
use std::string::ToString;
use std::net::{ SocketAddr, ToSocketAddrs };

use ::url::Url;

use super::{STUN_PORT, STUNS_PORT};

/// [RFC7064]:
///     https://tools.ietf.org/html/rfc7064
///     URI Scheme for the Session Traversal Utilities for NAT (STUN) Protocol

/// URI Scheme Syntax:
///     "stun" and "stuns" URIs have the following formal ABNF syntax
///     [RFC5234]:
///     stunURI       = scheme ":" host [ ":" port ]
///     scheme        = "stun" / "stuns"

/// Examples:
///     Table 1 shows examples for the "stun" and "stuns" URI schemes.  For
///     all these examples, the <host> component is populated with
///     "example.org".
///      +-----------------------+
///      | URI                   |
///      +-----------------------+
///      | stun:example.org      |
///      | stuns:example.org     |
///      | stun:example.org:8000 |
///      +-----------------------+


fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

pub fn url_parse (s: &str) -> Result<SocketAddr, &'static str> {
    let mut uri = s.to_owned();
    if uri.starts_with("stun") == false && uri.starts_with("stuns") == false {
        uri = format!("stun://{}", uri);
    }
    if uri.starts_with("stun:") && uri.starts_with("stun://") == false {
        uri = uri.replace("stun:", "stun://");
    } else if uri.starts_with("stun:") && uri.starts_with("stun://") == false {
        uri = uri.replace("stuns:", "stuns://");
    }
    match Url::parse(uri.as_ref()) {
        Ok(url) => {
            let scheme   = url.scheme();
            let host_str = url.host_str(); // Option
            let port     = url.port();     // Option

            if scheme != "stun" && scheme != "stuns" {
                return Err("scheme error");
            }
            if host_str.is_none() {
                return Err("host str error");
            }
            let port = match url.port() {
                Some(port) => port,
                None => match scheme {
                    "stun"  => STUN_PORT,
                    "stuns" => STUNS_PORT,
                    _       => unreachable!()
                }
            };

            
            let host_u8_vec: Vec<u8> = host_str.unwrap().split(".").into_iter().map(|x| x.parse::<u8>().unwrap()).collect();
            let host_u8_arr: [u8;4] = host_u8_vec.try_into().unwrap();
            let host_addr = SocketAddr::from((host_u8_arr, port));
            let mut loopup_host_iter = host_addr.to_socket_addrs().unwrap();
            let socket_addr = match loopup_host_iter.next() {
                Some(mut socket_addr) => {
                    socket_addr.set_port(port);
                    socket_addr
                },
                None => return Err("lookup host failure.")
            };
            Ok(socket_addr)
        },
        Err(_)  => Err("url parse error.")
    }
}

pub fn is_stun(){

}
pub fn is_stuns(){

}
