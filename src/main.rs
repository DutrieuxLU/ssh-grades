use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let tcp = TcpStream::connect("data.cs.purdue.edu:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("ldutrie", "Cubs2016@,PUSH").unwrap();

    println!("--- Testing directory change and ls ---");
    let mut channel = sess.channel_session().unwrap();
    let username = "ldutrie";

    let command = format!("cd ~/../cs354/grades/{} && ls -F", username);
    println!("Executing command: {}", command);
    channel.exec(&command).unwrap();

    // Read standard output
    let mut stdout_str = String::new();
    channel.read_to_string(&mut stdout_str).unwrap();

    // Read standard error
    let mut stderr_str = String::new();
    channel.stderr().read_to_string(&mut stderr_str).unwrap();

    if !stdout_str.is_empty() {
        println!("--- Standard Output ---\n{}", stdout_str);
    }
    if !stderr_str.is_empty() {
        println!("--- Standard Error ---\n{}", stderr_str);
        println!(" Email ldutrie@purdue.edu with the error output plz!");
    }

    channel.wait_close().unwrap();
    println!("Exit status: {}", channel.exit_status().unwrap());
}
