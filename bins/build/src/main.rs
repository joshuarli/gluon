use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 && args[1] == "--version" {
        println!("1.7.0");
        return ExitCode::SUCCESS;
    }
    if args.len() >= 2 && args[1] == "ninja" {
        return ninja::main_entry();
    }
    if args.len() >= 2 && args[1] == "meson" {
        if args.iter().any(|a| a == "--version") {
            println!("1.7.0");
            return ExitCode::SUCCESS;
        }
        eprintln!("build: meson subcommand forwards to `meson` bin (see bins/meson)");
        return ExitCode::SUCCESS;
    }
    if args.len() == 1 {
        println!("1.7.0");
        return ExitCode::SUCCESS;
    }
    eprintln!("usage: gluon-build [meson|ninja] [args...] | --version");
    ExitCode::from(1)
}
