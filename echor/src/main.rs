use clap::Command; //used to be App
use clap::{crate_version, Arg, ArgAction};

fn main() {
    let _matches = Command::new("echor")
        .version(crate_version!())
        .author("alex")
        .about("Rust echo")
        .arg(
            Arg::new("test")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<String> = _matches.get_many::<String>("test").unwrap().map(|v| v.to_string()).collect();
    let omit_newline = _matches.get_flag("omit_newline");
    
    print!("{}{}", text.join(" "), if omit_newline {""} else { "\n" });
}
