use clap::Parser;
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Parser)]
#[command(name = "password generator")]
#[command(about = "A password generator written in rust")]
struct Cli {
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    #[arg(short, long)]
    symbols: bool,

    #[arg(short, long)]
    numbers: bool,

    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn build_charset(symbols: bool, numbers: bool) -> Vec<char> {
    let mut chars: Vec<char> = ('a'..='z').chain('A'..='Z').collect();

    if numbers {
        chars.extend('0'..='9');
    }

    if symbols {
        chars.extend("!@#$%^&*()-_=+[]{}|;:,.<>?".chars());
    }

    chars
}

fn generate_password(length: usize, charset: &[char]) -> String {
    let mut rng = thread_rng();
    (0..length)
        .map(|_| charset.choose(&mut rng).unwrap())
        .collect()
}

fn main() {
    let args = Cli::parse();

    let charset = build_charset(args.symbols, args.numbers);

    println!("{}", "Password Generator".bold().red());
    println!("{}", "───────────".dimmed());

    for i in 1..=args.count {
        let password = generate_password(args.length, &charset);
        println!(
            "{} {}",
            format!("[{}]", i).dimmed(),
            password.green().bold()
        );
    }

    println!("{}", "─────────".dimmed());
    println!("{} {} chars", "Length:".dimmed(), args.length);
}
