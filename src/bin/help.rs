use std::env;

// Main help message
// =================
pub const HELP_MESSAGE : &str = r#"
ALL AVAILABLE COMMANDS:
======================
1. checkduplicates
2. askgpt

FOR MORE INFO ON A COMMAND, TYPE:
=================================
help <index>

EXAMPLE:
========
help 1
"#;

// Messages for each command    
// ==========================
pub const CHECKDUPLICATES_MESSAGE : &str = r#"
checkduplicates: checks for duplicate files in a directory
USAGE:
======
checkduplicates <directory>

EXAMPLE:
========
checkduplicates ~/Downloads
"#;


pub const ASKGPT_MESSAGE : &str = r#"
askgpt: asks questions to gpt in the terminal
USAGE:
======
askgpt <question>

EXAMPLE:
========
askgpt how to find and replace in vim

NOTE:
=====
1. This command is optimised to answer questions about programming succintly.
2. Do not include ? in the question.
"#;

// Error handling messages
pub const INDEX_ERROR_MESSAGE : &str = r#"
ERROR: Invalid index
====================
Make sure that your index is within the range of the available commands.
Enter help to see the list of available commands and their indices.
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", args[0]);
        // check if args[0] contains help
        // we can't use args[0] == "help" because args[0] is the path to the executable
        if args[0].contains("help") {
            println!("{}", HELP_MESSAGE);
        }
    } else {
        match args[1].parse::<usize>() {
            Ok(number) => match number {
                1 => println!("{}", CHECKDUPLICATES_MESSAGE),
                2 => println!("{}", ASKGPT_MESSAGE),
                _ => println!("{}", INDEX_ERROR_MESSAGE),
            },
            Err(_) => println!("{}", INDEX_ERROR_MESSAGE),
        }
    }
}
