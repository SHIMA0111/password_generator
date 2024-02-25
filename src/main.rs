use crate::password_generator::{Args, generate_password};
use clap::{Command, Arg};

mod password_generator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("password_generator")
        .version("0.1.0")
        .author("SHIMA <shima@little-tabby.com>")
        .about("Password generator following CSPRNG.")
        .arg(
            Arg::new("length")
                .long("length")
                .short('l')
                .help("Length of the password")
                .default_value("12")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("number-of-passwords")
                .long("number-of-passwords")
                .short('n')
                .help("Number of the generating passwords")
                .default_value("1")
                .value_parser(clap::value_parser!(usize))
        )
        .arg(
            Arg::new("exclude-symbols")
                .long("exclude-symbols")
                .short('e')
                .help("Symbols list you want exclude the password generation: Default\"`~!@#$%^&*()_+-={}[]\\|:;\"'<>,.?/\"")
                .default_value("")
                .value_parser(clap::value_parser!(String))
        )
        .arg(
            Arg::new("")
        )
        .get_matches();

    let length = matches.get_one::<usize>("length")
        .ok_or_else(|| Box::new(
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "\"length\" parameter cannot be received.")))?;

    let number_of_passwords = matches.get_one::<usize>("number-of-passwords")
        .ok_or_else(|| Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "\"number_of_passwords\" parameter cannot be received.")))?;
    let exclude_symbols = matches.get_one::<String>("exclude-symbols")
        .ok_or_else(|| Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "\"exclude_symbols\" parameter cannot be received."
        )))?;
    let exclude_symbols_vec: Vec<char> = exclude_symbols.chars().collect();

    let arg = Args::new(*length, *number_of_passwords, exclude_symbols_vec);
    let passwords = generate_password(&arg)?;
    for password in passwords {
        println!("{}", password);
    }

    Ok(())
}
