use crate::*;

pub fn read_files(file_list: &mut Vec<GradeFile>, grades_dir: &str, sess: Session) {
    for file in file_list {
        // 1. Open a new channel for each command
        let mut channel = sess.channel_session().unwrap();
        let command = format!("cd {} && cat {}", grades_dir, file.name);

        channel.exec(&command).unwrap();

        let mut content = String::new();
        channel.read_to_string(&mut content).unwrap();

        file.content = Some(content);
        // 4. Print the output
        println!("\n--- Successfully Read {} ---", file.name.bold());
        // 5. Close the channel and check for errors
        channel.wait_close().unwrap();
        let exit_code = channel.exit_status().unwrap();
        if exit_code != 0 {
            println!(
                "[Warning] Command for {} exited with status {}",
                file.name, exit_code
            );
        }
    }
}

pub fn parse_grades(file_list: &mut Vec<GradeFile>, user: User) {
    for file in file_list {
        let curr_content = file.content.as_ref().unwrap();
        // if the file only has the username of the current user, that means that file has not
        // yet been filled in.
        if curr_content.trim() == user.user_name {
            continue;
        }
        file.has_content = true;
        // all files that will be worked with from here on HAVE been graded.
        let mut grade_str = curr_content.split_whitespace().last().unwrap().split('/');
        file.points_gained = Some(grade_str.next().unwrap().parse().unwrap());
        file.points_available = Some(grade_str.next().unwrap().parse().unwrap());
    }
}
