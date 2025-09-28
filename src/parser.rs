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
        file.checked = true;
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

pub fn parse_grades(file_list: &mut Vec<GradeFile>) {
    for file in file_list {
        if file.checked {
            let curr_content = file.content.as_ref().unwrap();
            match curr_content.trim() {
                "ldutrie" => {
                    println!("File {} is empty", file.name.red());
                    continue;
                }
                _ => println!("File {} Has been graded", file.name.bright_green()),
            }
            // all files that will be worked with from here on HAVE been graded.
            let mut grade_str = curr_content.split_whitespace().last().unwrap().split('/');
            file.points_gained = Some(grade_str.next().unwrap().parse().unwrap());
            file.points_available = Some(grade_str.next().unwrap().parse().unwrap());
            // println!(
            //     "Grade for {}, {}",
            //     file.name,
            //     file.points_gained.unwrap() / file.points_available.unwrap()
            // );
        }
    }
}
