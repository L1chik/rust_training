// use std::io::prelude::*;
use std::net::{TcpListener};
// use bufstream::BufStream;
// use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}
// fn make_directory(param: &str) -> String {
//     match fs::create_dir_all(param) {
//         Ok(_) => String::from("Success"),
//         Err(err) => err.to_string(),
//     }
// }
//
// fn get_file_list() -> String {
//     let mut listing = String::with_capacity(8192);
//
//     for file in fs::read_dir(".").unwrap() {
//         let entry = file.unwrap().path().display().to_string();
//         listing.push_str(entry.as_str());
//     }
//
//     listing
// }
//
// fn handle_req(conn: TcpStream) {
//     let mut req = String::with_capacity(512);
//     let mut response = String::with_capacity(4096);
//     let mut reader = BufStream::new(&conn);
//
//     match reader.write(b">") {
//         Ok(_) => (),
//         Err(err) => println!("Received an error on write! {}", err),
//     }
//
//     let size = reader.read_line(&mut req);
//
//     if size.unwrap() > 0 {
//         let mut params = req.split_whitespace();
//         let command = params.next().unwrap();
//
//         match command {
//             "flist" => response = get_file_list(),
//             "md" => response = make_directory(params.next().unwrap()),
//             _ => response = String::from("Unacceptable command")
//         }
//
//         match reader.write(&response.into_bytes()) {
//             Ok(_) => (),
//             Err(err) => println!("Received an error on write! {}", err)
//         };
//     }
// }


// fn main() -> std::io::Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:7878")?;
//
//     for stream in listener.incoming() {
//         handle_req(stream?);
//     }
//
//     Ok(())
// }



fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send msg to rx");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}