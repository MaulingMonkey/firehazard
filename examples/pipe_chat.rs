use firehazard::access::{GENERIC_READ, GENERIC_WRITE};
use firehazard::prelude::*;

use winapi::um::fileapi::OPEN_EXISTING;

use std::io::*;
use std::process::exit;
use std::sync::Mutex;
use std::thread;

const DEFAULT_USERNAME              : &'static str                      = "Default";
//const SERVER_TO_CLIENT_PIPE_NAME    : abistr::CStrNonNull<'static, u16> = abistr::cstr16!(r"\\.\pipe\firehazard-pipe_chat-server-to-client");
//const CLIENT_TO_SERVER_PIPE_NAME    : abistr::CStrNonNull<'static, u16> = abistr::cstr16!(r"\\.\pipe\firehazard-pipe_chat-client-to-server");
const SERVER_TO_CLIENT_PIPE_NAME    : &'static str                      = r"\\.\pipe\firehazard-pipe_chat-server-to-client";
const CLIENT_TO_SERVER_PIPE_NAME    : &'static str                      = r"\\.\pipe\firehazard-pipe_chat-client-to-server";
const SERVER_TO_CLIENT_BUFFER_SIZE  : u32                               = 4 << 10; // 4 KiB
const CLIENT_TO_SERVER_BUFFER_SIZE  : u32                               = 4 << 10; // 4 KiB



fn main() {
    let mut args = std::env::args();
    let _exe = args.next();
    let subcommand = args.next().unwrap_or_else(||{
        eprintln!("Usage:");
        eprintln!("    pipe_chat server");
        eprintln!("    pipe_chat client UserName");
        exit(1);
    });
    match subcommand.as_str() {
        "client" => client(args),
        "server" => server(args),
        _ => {
            eprintln!("error: unexpected subcommand: {subcommand:?}");
            exit(1);
        },
    }
}



fn client(mut args: std::env::Args) {
    let username = args.next().unwrap_or_else(||{
        print!("Enter a username: ");
        let _ = stdout().flush();
        let mut username = String::new();
        let _ = stdin().read_line(&mut username);
        if username.is_empty() { username.push_str(DEFAULT_USERNAME); }
        username
    });
    let username = username.trim();

    if let Some(extra) = args.next() {
        eprintln!("unexpected arguments after `pipe_chat client {username}`: {extra:?}");
        exit(1);
    }

    // Safety: sound as we've avoided FILE_FLAG_OVERLAPPED here
    let server_to_client = unsafe { pipe::sync::OwnedReader::from_raw_nn(create_file_w(SERVER_TO_CLIENT_PIPE_NAME, GENERIC_READ,  None, None, OPEN_EXISTING, 0, None).unwrap().into_handle_nn()) };
    let client_to_server = unsafe { pipe::sync::OwnedWriter::from_raw_nn(create_file_w(CLIENT_TO_SERVER_PIPE_NAME, GENERIC_WRITE, None, None, OPEN_EXISTING, 0, None).unwrap().into_handle_nn()) };
    thread::scope(|s|{
        thread::Builder::new().name(format!("pipe reading thread")).spawn_scoped(s, ||{
            let mut pipe = BufReader::new(&server_to_client);
            let mut line = String::new();
            loop {
                line.clear();
                if pipe.read_line(&mut line).is_err() { break }
                println!("{line}");
            }
        }).unwrap();
        thread::Builder::new().name(format!("stdin reading thread")).spawn_scoped(s, ||{
            let mut pipe = &client_to_server;
            let mut line = String::new();
            loop {
                line.clear();
                line.push_str("<");
                line.push_str(username);
                line.push_str("> ");
                if stdin().read_line(&mut line).is_err() { break }
                if pipe.write_all(line.as_bytes()).is_err() { break }
            }
        }).unwrap();
    });
}



fn server(mut args: std::env::Args) {
    if let Some(extra) = args.next() {
        eprintln!("unexpected arguments after `pipe_chat server`: {extra:?}");
        exit(1);
    }

    thread::scope(|s|{
        static SERVER_TO_CLIENT_PIPES : Mutex<Vec<pipe::named::Connected>> = Mutex::new(Vec::new());

        for no in 1 ..= 4 {

            // client to server

            let pipe = create_client_to_server_pipe(no == 1);
            thread::Builder::new().name(format!("client to server pipe listener #{no}")).spawn_scoped(s, move || {
                let mut pipe = pipe;
                loop {
                    let mut accepted = pipe.accept().unwrap();
                    thread::Builder::new().name(format!("client to server connection")).spawn_scoped(s, move || {
                        let mut line = String::new();
                        let mut reader = BufReader::new(accepted.bytes());
                        while let Ok(n) = reader.read_line({ line.clear(); &mut line }) {
                            if n == 0 { break }

                            println!("{line}");
                            SERVER_TO_CLIENT_PIPES.lock().unwrap().retain_mut(|pipe|{
                                pipe.write_all(line.as_bytes()).is_ok()
                            });
                        }
                    }).unwrap();
                    pipe = create_client_to_server_pipe(false);
                }
            }).unwrap();

            fn create_client_to_server_pipe(first: bool) -> pipe::named::Listener {
                pipe::named::create(
                    CLIENT_TO_SERVER_PIPE_NAME,
                    pipe::ACCESS_INBOUND | (u32::from(first) * file::FLAG_FIRST_PIPE_INSTANCE),
                    pipe::REJECT_REMOTE_CLIENTS,
                    pipe::UNLIMITED_INSTANCES,
                    CLIENT_TO_SERVER_BUFFER_SIZE,
                    0,
                    pipe::NMPWAIT::USE_DEFAULT_WAIT,
                    None
                ).unwrap()
            }

            // server to client

            let pipe = create_server_to_client_pipe(no == 1);
            thread::Builder::new().name(format!("server to client pipe listener #{no}")).spawn_scoped(s, move || {
                let mut pipe = pipe;
                loop {
                    let accepted = pipe.accept().unwrap();
                    SERVER_TO_CLIENT_PIPES.lock().unwrap().push(accepted);
                    pipe = create_server_to_client_pipe(false);
                }
            }).unwrap();

            fn create_server_to_client_pipe(first: bool) -> pipe::named::Listener {
                pipe::named::create(
                    SERVER_TO_CLIENT_PIPE_NAME,
                    pipe::ACCESS_OUTBOUND | (u32::from(first) * file::FLAG_FIRST_PIPE_INSTANCE),
                    pipe::REJECT_REMOTE_CLIENTS,
                    pipe::UNLIMITED_INSTANCES,
                    0,
                    SERVER_TO_CLIENT_BUFFER_SIZE,
                    pipe::NMPWAIT::USE_DEFAULT_WAIT,
                    None
                ).unwrap()
            }

        }
    });
}
