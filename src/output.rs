use crate::*;

pub fn prompt_user() {}

fn colorize_grade(grade_as_pct: f64) -> colored::ColoredString {
    if grade_as_pct > 90.0 {
        format!("{:.2}", grade_as_pct).bright_green()
    } else if grade_as_pct > 80.0 {
        format!("{:.2}", grade_as_pct).blue()
    } else if grade_as_pct > 70.0 {
        format!("{:.2}", grade_as_pct).yellow()
    } else if grade_as_pct > 60.0 {
        format!("{:.2}", grade_as_pct).magenta()
    } else {
        format!("{:.2}", grade_as_pct).bright_red()
    }
}

pub fn print_summary(grade_list: &Vec<GradeFile>) {
    let mut total_points_gained = 0.0;
    let mut total_points_available = 0.0;

    for proj in grade_list {
        if proj.has_content {
            let curr_grade = (proj.points_gained.unwrap() / proj.points_available.unwrap()) * 100.0;
            total_points_gained += proj.points_gained.unwrap();
            total_points_available += proj.points_available.unwrap();
            println!(
                "File {} has been graded. Grade => {} ",
                proj.name.bright_green(),
                colorize_grade(curr_grade)
            );
        } else {
            println!("File {} is empty", proj.name.red());
        }
    }

    let grade_as_pct = (total_points_gained / total_points_available) * 100.0;
    let colored_grade = colorize_grade(grade_as_pct);

    println!("--- OVERALL GRADE ---");
    println!("--------{}--------", colored_grade);
    println!("---------------------");
}
