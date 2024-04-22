use std::{env, error::Error, process};

pub fn main() -> Result<(), Box<dyn Error>> {
    const HELP: &str = include_str!("help.txt");

    let mut args = env::args().skip(1);
    match args.next().as_ref().map(String::as_str) {
        Some("build") => freight::build()?,
        Some("test") => {
            freight::build_tests()?;
            args.by_ref()
                .take_while(|arg| *arg != "--")
                .for_each(|_| {});
            freight::run_tests(args.collect::<Vec<String>>())?
        }
        Some("help") => println!("{HELP}"),
        _ => {
            println!("Unsupported command\n{HELP}");
            process::exit(1);
        }
    }
    Ok(())
}
