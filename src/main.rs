#! [allow(dead_code)]
#! [allow(unused_variables)]
#! [allow(unused_assignments)]

use tokio::net::TcpListener;
use tokio::prelude::*;


mod j_read;
mod j_write;

fn main() {
    let addr = "127.0.0.1:8088".parse().unwrap();
    let lis = TcpListener::bind(&addr).expect("Could not bind address");

        let fut = lis.incoming().for_each(|sock| {
            let (sock_r, sock_w) = sock.split();
            let rd = j_read::JRead::new(sock_r).for_each(|s| {
                println!("Recieved: {}", s);
            Ok(())
            }).map_err(|_|());
            tokio::spawn(rd);
            Ok(())
        }).map_err(|e| println!("Listening Err :{:?}", e ));

        tokio::run(fut);

        println!("Hello World");
}

