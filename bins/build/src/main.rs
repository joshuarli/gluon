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
        let mut synthetic = vec!["meson".to_string()];
        synthetic.extend(args[2..].iter().cloned());
        let cli = meson::Cli::parse_args_from(synthetic);
        return ExitCode::from(cli.run() as u8);
    }
    if args.len() == 1 {
        println!("1.7.0");
        return ExitCode::SUCCESS;
    }
    eprintln!("usage: gluon-build [meson|ninja] [args...] | --version");
    ExitCode::from(1)
}
