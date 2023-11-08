use std::env;

// Main help message
// =================
pub const HELP_MESSAGE : &str = r#"
TO SEE COMMONLY USED COMMANDS, TYPE:
====================================
help others

ALL AVAILABLE CUSTOM COMMANDS:
======================
1. checkduplicates
2. askgpt

FOR MORE INFO ON A COMMAND, TYPE:
=================================
help <index>
OR
help <command>

EXAMPLE:
========
help 1
OR
help checkduplicates
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
// =======================
pub const INDEX_ERROR_MESSAGE : &str = r#"
ERROR: Invalid index
====================
Make sure that your index is within the range of the available commands.
Enter help to see the list of available commands and their indices.
"#;

// Common commands
// ===============
pub const LSOF_MESSAGE : &str = r#"
List all processes running on a port
lsof -i :<port>
"#;

pub const WHICH_MESSAGE : &str = r#"
Find the path of a command
which <command>
"#;

pub const KILL_MESSAGE : &str = r#"
Kill a process
kill -9 <pid>
"#;

pub const CHMOD_MESSAGE : &str = r#"
Give executable permissions to a file
chmod x <file>
"#;

pub const JAVAFX_MESSAGE : &str = r#"
Run a jar file with JavaFX
java --module-path /Users/jcjustin/Downloads/javafx-sdk-17.0.8/lib --add-modules=javafx.controls,javafx.fxml -jar <jarfile>
"#;

pub const CARGO_RUN_MESSAGE : &str = r#"
Run a rust file
cargo run --bin <file>(without .rs)
"#;

pub const CARGO_BUILD_MESSAGE : &str = r#"
Build all rust files
cargo build --release
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // check if args[0] contains help
        // we can't use args[0] == "help" because args[0] is the path to the executable
        if args[0].contains("help") {
            println!("{}", HELP_MESSAGE);
        }
    } else {
        match args[1].as_str() {
                "1" | "checkduplicates" => println!("{}", CHECKDUPLICATES_MESSAGE),
                "2" | "askgpt" => println!("{}", ASKGPT_MESSAGE),
                "others" => print_common_commands(), 
                _ => println!("{}", INDEX_ERROR_MESSAGE),
        }
    }
}

fn print_common_commands() {
    let common_commands = vec![LSOF_MESSAGE, WHICH_MESSAGE, KILL_MESSAGE, CHMOD_MESSAGE, JAVAFX_MESSAGE];
    for command in common_commands {
        println!("{}", command);
    }
}

