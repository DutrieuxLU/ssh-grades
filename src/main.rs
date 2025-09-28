use colored::Colorize;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Debug)]
pub struct GradeFile {
    name: String,
    content: Option<String>,
    points_gained: Option<f64>,
    points_available: Option<f64>,
    has_content: bool,
}
pub struct User {
    user_name: String,
    password: String,
}

pub mod output;
pub mod parser;
pub mod user;
fn main() {
    let curr_user = user::get_user()
        .expect("Error in User getting. Try Restarting the program and email ldutrie@purdue.edu");

    let tcp = TcpStream::connect("data.cs.purdue.edu:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(curr_user.user_name.as_str(), curr_user.password.as_str())
        .unwrap();

    println!("--- Testing directory change and ls ---");
    let mut channel = sess.channel_session().unwrap();
    let remote_base_dir = format!("~/../cs354/grades/{}", curr_user.user_name);
    let command = format!("cd {} && ls -F", remote_base_dir);
    println!("Executing command: {}", command);
    channel.exec(&command).unwrap();

    // Read standard output
    let mut stdout_str = String::new();
    channel.read_to_string(&mut stdout_str).unwrap();
    let mut file_list: Vec<GradeFile> = stdout_str
        .split_whitespace()
        .map(|s| GradeFile {
            name: (s.to_string()),
            content: (None),
            points_gained: (None),
            points_available: (None),
            has_content: (false),
        })
        .collect();

    parser::read_files(&mut file_list, &remote_base_dir, sess);

    // Read standard error
    let mut stderr_str = String::new();
    channel.stderr().read_to_string(&mut stderr_str).unwrap();

    if !stderr_str.is_empty() {
        println!("--- Standard Error ---\n{}", stderr_str);
        println!(" Email ldutrie@purdue.edu with the error output plz!");
    }
    channel.wait_close().unwrap();
    println!("Exit status: {}", channel.exit_status().unwrap());

    parser::parse_grades(&mut file_list, curr_user);
    output::print_summary(&file_list);
}
