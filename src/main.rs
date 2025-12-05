use std::env;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let day = match args.next() {
        Some(d) => format!("{:0>2}", d), // Pad with zeroes to length 2
        None => {
            eprintln!("Usage: cargo run -- <day>");
            std::process::exit(1);
        }
    };
    println!("Running challenge for day {}", day);
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let exe_dir = current_exe.parent().expect("Failed to get executable directory");

    let binary_path = exe_dir.join(format!("day{}", day));
    if !binary_path.exists() {
        eprintln!("Challenge binary for day {} does not exist at {:?}", day, binary_path);
        std::process::exit(1);
    }

    let status = Command::new(binary_path)
        .status()
        .expect("Failed to execute challenge");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
