use firehazard::prelude::*;

use std::io::*;
use std::process::exit;
use std::sync::Mutex;
use std::thread;

const DEFAULT_USERNAME              : &'static str                      = "Default";
const SERVER_TO_CLIENT_PIPE_NAME    : abistr::CStrNonNull<'static, u16> = abistr::cstr16!(r"\\.\pipe\firehazard-pipe_chat-server-to-client");
const CLIENT_TO_SERVER_PIPE_NAME    : abistr::CStrNonNull<'static, u16> = abistr::cstr16!(r"\\.\pipe\firehazard-pipe_chat-client-to-server");
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

    let server_to_client = pipe::sync::read_existing(SERVER_TO_CLIENT_PIPE_NAME).unwrap();
    let client_to_server = pipe::sync::write_existing(CLIENT_TO_SERVER_PIPE_NAME).unwrap();
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

    // TODO: make logon_sid return a wrapper which is less gross? and returns an error even if windows doesn't if GroupCount != 1?
    let token = open_process_token(get_current_process(), token::QUERY).unwrap();
    let logon = token.logon_sid().unwrap();
    let logon = logon.groups()[0].sid;

    let rev = acl::REVISION; // TODO: make acl::REVISION nonsense more internal?  choice between it and REVISION_DS is if object guids are present...?
    let mut dacl = acl::Builder::new(rev);
    dacl.add_access_allowed_ace(rev, file::GENERIC_READ | file::GENERIC_WRITE, logon).unwrap(); // If the client needs to open the pipe for read or write, the server needs the other, even though it's creating the pipe.
    dacl.finish().unwrap(); // TODO: gross
    let security_descriptor = security::DescriptorBuilder::new().dacl(&mut dacl, false).unwrap().finish();
    let security_attributes = security::Attributes::new(Some(&security_descriptor), false);

    thread::scope(|s|{
        static SERVER_TO_CLIENT_PIPES : Mutex<Vec<pipe::named::Connected>> = Mutex::new(Vec::new());

        for no in 1 ..= 4 {

            // client to server

            let pipe = create_client_to_server_pipe(no == 1, &security_attributes);
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
                    pipe = create_client_to_server_pipe(false, &security_attributes);
                }
            }).unwrap();

            fn create_client_to_server_pipe(first: bool, security_attributes: &security::Attributes) -> pipe::named::Listener {
                pipe::named::create_w(
                    CLIENT_TO_SERVER_PIPE_NAME,
                    pipe::ACCESS_INBOUND | (u32::from(first) * file::FLAG_FIRST_PIPE_INSTANCE),
                    pipe::REJECT_REMOTE_CLIENTS,
                    pipe::UNLIMITED_INSTANCES,
                    CLIENT_TO_SERVER_BUFFER_SIZE,
                    0,
                    pipe::NMPWAIT::USE_DEFAULT_WAIT,
                    Some(security_attributes)
                ).unwrap()
            }

            // server to client

            let pipe = create_server_to_client_pipe(no == 1, &security_attributes);
            thread::Builder::new().name(format!("server to client pipe listener #{no}")).spawn_scoped(s, move || {
                let mut pipe = pipe;
                loop {
                    let accepted = pipe.accept().unwrap();
                    SERVER_TO_CLIENT_PIPES.lock().unwrap().push(accepted);
                    pipe = create_server_to_client_pipe(false, &security_attributes);
                }
            }).unwrap();

            fn create_server_to_client_pipe(first: bool, security_attributes: &security::Attributes) -> pipe::named::Listener {
                pipe::named::create_w(
                    SERVER_TO_CLIENT_PIPE_NAME,
                    pipe::ACCESS_OUTBOUND | (u32::from(first) * file::FLAG_FIRST_PIPE_INSTANCE),
                    pipe::REJECT_REMOTE_CLIENTS,
                    pipe::UNLIMITED_INSTANCES,
                    0,
                    SERVER_TO_CLIENT_BUFFER_SIZE,
                    pipe::NMPWAIT::USE_DEFAULT_WAIT,
                    Some(security_attributes)
                ).unwrap()
            }

        }
    });
}
