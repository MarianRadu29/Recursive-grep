mod helpers;
use helpers::*;

//Let -> un exemplu pentru a cauta case insensitive

fn main() {
    let mut path = String::new();
    let mut input;

    'lloop: loop {
        print!("{}:", PROMPT.green().bold());
        io::stdout().flush().expect("Error unblocking stdout");
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from stdin");
        input = input.trim().to_string();
        if input==String::new(){
            continue;
        }

        let mut args = input.split_whitespace().collect::<Vec<&str>>();

        if args[0] == "exit" {
            break;
        }
        if args[0] == "help" {
            help();
            continue;
        }
        if args[0] == "cls" {
            match clearscreen::clear() {
                Ok(()) => {}
                Err(e) => println!("{}", format!("Error clear screen: {}", e).bold().red()),
            };
            continue;
        }
        if args[0] != "setpath" && args[0] != "rgrep" {
            println!("{}", "Command is not found".red().bold());
            help();
            continue;
        }

        if args[0] == "setpath" && args.len() == 1 {
            println!(
                "usage: {0} [\"{1}\" | {1}]",
                "setpath".yellow().bold(),
                "path".bold().blue()
            );
            continue;
        }
        if args[0] == "setpath" {
            path = if args[1].starts_with("\"") && args[1].ends_with("\"") {
                let mut p = args[1].to_string();
                p.remove(0);
                p.remove(p.len() - 1);
                p
            } else {
                args[1].to_string()
            };
            let check_path = std::path::Path::new(&path);
            if check_path.exists() {
                println!("The path is configured!");
            } else {
                println!(
                    "{}",
                    "The path given at input doesn't exist in the work directory"
                        .red()
                        .bold()
                );
                path = String::new();
            }
            continue;
        }

        if args[0] == "rgrep" && args.len() == 1 {
            print_info_rgrep();
            continue;
        }
        if args[args.len() - 1].starts_with("\"") && args[args.len() - 1].ends_with("\"") {
            if let Some(last) = args.last_mut() {
                *last = last.trim_matches('\"');
            }
        }

        if args.len() > 2
            && !args
                .iter()
                .skip(1)
                .take(args.len() - 2)
                .collect::<Vec<&&str>>()
                .iter()
                .all(|&&x| x.starts_with("-"))
        {
            println!("{}", "invalid select option".red().bold());
            print_info_rgrep();
            continue;
        }

        let mut op = OptionCommand::new();
        let pattern = args[args.len() - 1].to_string();
        let re;

        for arg in args.iter().take(args.len() - 1).skip(1) {
            if arg.starts_with("-") {
                let mut option = arg.to_string();
                option.remove(0);
                match option.as_str() {
                    "i" => op.ignore_case = true,
                    "c" => op.only_print_count_match = true,
                    "r" => op.regex_search = true,
                    nr if nr.parse::<usize>().is_ok() => {
                        op.max_number_of_lines = nr.parse::<usize>().unwrap() //stim din conditia din if ca putem face parsarea
                    }
                    e => {
                        let text = format!("Option \"-{}\" doesn't exist", e);
                        println!("{}", text.red().bold());
                        print_info_rgrep();
                        continue 'lloop;
                    }
                }
            }
        }

        let re_result = if op.ignore_case {
            RegexBuilder::new(&pattern).case_insensitive(true).build()
        } else {
            RegexBuilder::new(&pattern).build()
        };

        if path == String::new() {
            println!("{}", "Path is not found!!".red().bold());
            println!(
                "usage: {0} [\"{1}\" | {1}]",
                "setpath".yellow().bold(),
                "path".blue().bold()
            );
            continue;
        }

        match re_result {
            Ok(regex) => re = regex,
            Err(e) => {
                let text = format!("Error creating regex: {}", e);
                println!("{}", text.bold().red());
                continue;
            }
        }
        let mut count_found = 0usize; //pentru optiunea "-c"
    
        WalkDir::new(path.as_str())
        .into_iter()
        .filter_map(|x| match x {
            Ok(a) => Some(a),
            Err(e) => {
                println!("{e:?}");
                None
            }
        })
        .filter(|x| x.file_type().is_file())
        .for_each(|entry|{
            if count_found == op.max_number_of_lines {
                return;
            }
            if let Err(e) = process_file(&re, entry, &mut count_found, &op) {
                let text = format!(
                    "Error retrieving certain files from the source folder: {}",
                    e
                );
                println!("{}", text.bold().red());
            }
            println!();
            
        }
        )
    
    }
}
