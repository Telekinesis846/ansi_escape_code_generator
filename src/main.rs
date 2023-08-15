use std::io;

fn main() {
    let styles = 0..8;
    let font_colour = [30..37, 90..97];
    let highlight_colour = [40..47, 100..107];

    println!(r"What escape sequence do you want to use? (1: \x1b 2: \033 3: \e)");
    let sequence = input();

    println!(r"For the foreground colour, do you want to use RGB color? (1: Yes 2: No)");
    print!("\x1b[6m BOLD")
}

fn create_ansi_code(escape_sequence: &str, styles: [i32; 3]) -> String {
    let values = styles
        .iter()
        .map(|x| x.to_string() + ";")
        .collect::<String>();

    return format!("{}[{}m", escape_sequence, values);
}

fn input() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => return user_input.trim().to_string(),
        Err(_) => {
            eprintln!("Error reading input.");
            return String::new(); // Return an empty string on error
        }
    }
}
