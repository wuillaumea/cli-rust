use clap::Command; //used to be App
use clap::{crate_version, Arg, ArgAction};

fn main() {
    let _matches = Command::new("echor")
        .version(crate_version!())
        .author("alex")
        .about("Rust echo")
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("test")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .get_matches();

    // cast the <option<T>> to a String, unwrap the option, then copy the iter of string references to values, then collect
    // instead of .map(|v| v.to_string()) you can use cloned to convert from list of references to values
    let text: Vec<String> = _matches.get_many::<String>("test").unwrap().cloned().collect();
    let omit_newline = _matches.get_flag("omit_newline");
    
    print!("{}{}", text.join(" "), if omit_newline {""} else { "\n" });
}
