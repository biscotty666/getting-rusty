#[derive(Debug)] // so we can format with {:?} in println!
struct Arguments {
    target: String,
    replacment: String,
    filename: String,
    output: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!("{} - change occurrences of one string into another",
            "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacment> <INPUT> <OUTPUT>");
}

use std::env;

fn parse_args() -> Arguments {

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} wrong number of arguments: expected 4, got {}",
                    "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacment: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone()
    }
}

use regex::Regex;
fn replace(target: &str, replacment: &str, text: &str)
     -> Result<String, regex::Error>
{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacment).to_string())
}


use std::fs;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}': {:?}",
                "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacment, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}",
                        "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, &replaced_data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to write to file '{}': {:?}",
                        "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };
}

/*
quickreplace on  main [?] is 📦 v0.1.0 via 🦀 v1.83.0-beta.6 via ❄  impure (nix-shell-env)
❯ cargo run "world" "Rust" test.txt test-modified.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/quickreplace world Rust test.txt test-modified.txt`

quickreplace on  main [?] is 📦 v0.1.0 via 🦀 v1.83.0-beta.6 via ❄  impure (nix-shell-env)
❯ \cat test.txt
Hello, world

quickreplace on  main [?] is 📦 v0.1.0 via 🦀 v1.83.0-beta.6 via ❄  impure             ❯ \cat test-modified.txt
Hello, Rust

❯ cargo run "[[a-z]" "0" test.txt test-modified.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/quickreplace '[[a-z]' 0 test.txt test-modified.txt`
Error: failed to replace text: Syntax(
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
regex parse error:
    [[a-z]
    ^
error: unclosed character class
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
)
*/
