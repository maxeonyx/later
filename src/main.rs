fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: later <file.later>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let _source = match std::fs::read_to_string(filename) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", filename, e);
            std::process::exit(1);
        }
    };

    // TODO: Implement lexer, parser, interpreter
    eprintln!("later: not yet implemented");
    std::process::exit(1);
}
