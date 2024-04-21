use clap::{Arg, ArgAction, Command};

fn main() {
    let _matches = Command::new("echor")
        .version("0.1.0")
        .author("Russ Gunther <not-posting-my-email-publically@gmail.com>")
        .about("Rust version of echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    let text: Vec<String> = _matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = _matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{ending}", text.join(" "))
}
