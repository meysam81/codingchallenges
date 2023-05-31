use std::collections::HashMap;
use std::io::Read;

struct Counter {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

fn counter(contents: String) -> Counter {
    Counter {
        lines: contents.lines().count(),
        words: contents.split_whitespace().count(),
        bytes: contents.len(),
        chars: contents.chars().count(),
    }
}

#[derive(Debug, Default)]
struct Config {
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

impl Config {
    fn from(options: &[char]) -> Self {
        let mut config = Config::default();
        options.iter().for_each(|option| match option {
            'c' => {
                config.bytes = true;
            }
            'w' => {
                config.words = true;
            }
            'l' => {
                config.lines = true;
            }
            'm' => {
                config.chars = true;
            }
            _ => {
                eprintln!("option `{}` not supported", option);
            }
        });

        if !config.lines && !config.words && !config.bytes && !config.chars {
            config.lines = true;
            config.words = true;
            config.bytes = true;
        }
        config
    }
}

fn make_printable(counter: Counter, config: &Config) -> String {
    let mut to_print = String::new();
    if config.lines {
        to_print.push_str(&format!("  {}", counter.lines));
    }
    if config.words {
        to_print.push_str(&format!("  {}", counter.words));
    }
    if config.chars {
        to_print.push_str(&format!("  {}", counter.chars));
    }
    if config.bytes {
        to_print.push_str(&format!("  {}", counter.bytes));
    }
    to_print
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // the first argument is the path to the executable
    let args = &args[1..];
    let mut result: HashMap<String, Counter> = HashMap::new();

    if args.is_empty() {
        eprintln!("usage: wc [-l] [-m] [-c] [-w] <file>...");
        std::process::exit(1);
    }
    let options: Vec<_> = args
        .iter()
        .filter(|arg| arg.starts_with('-') && !arg.starts_with("--"))
        .map(|arg| &arg[1..])
        .flat_map(|arg| arg.chars())
        .collect();
    let config = Config::from(&options);

    let files: Vec<_> = args.iter().filter(|arg| !arg.starts_with('-')).collect();
    if files.is_empty() {
        let mut contents = String::new();
        std::io::stdin().read_to_string(&mut contents).unwrap();
        result.insert("stdin".to_string(), counter(contents));
        result.into_iter().for_each(|(_file, count)| {
            let to_print = make_printable(count, &config);
            println!("{}", to_print);
        });
        std::process::exit(0);
    }

    files.iter().for_each(|file| {
        let contents = std::fs::read_to_string(file).unwrap();
        let count = counter(contents);
        result.insert(file.to_string(), count);
    });

    result.into_iter().for_each(|(file, count)| {
        let mut to_print = make_printable(count, &config);
        to_print.push_str(&format!("  {}", file));
        println!("{}", to_print);
    });
}
