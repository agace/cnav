mod types;
mod search;
mod ui;
mod open;

use clap::{Arg, Command as ClapCommand};
use crate::search::search_symbol;
use crate::open::open_in_editor;

fn main() {
    let matches = ClapCommand::new("cnav")
        .version("0.1.0")
        .author("Bryan Charles bryancharlesdnz@gmail.com")
        .about("Navigate code at terminal velocity. An interactive bridge between ripgrep and your terminal editor.")

        // flags
        .arg(
            Arg::new("no-interactive")
                .short('n')
                .long("no-interactive")
                .help("Open the first match directly")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("tmux")
                .short('t')
                .long("tmux")
                .help("Open in tmux split if inside tmux")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("editor")
                .short('e')
                .long("editor")
                .help("Specify the terminal editor to use (default: vim)")
                .num_args(1)
        )
        // positional arguments
        .arg(
            Arg::new("path")
                .help("Path to search in")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("symbol")
                .help("Symbol (function, variable, class) to search for")
                .required(true)
                .index(2),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let symbol = matches.get_one::<String>("symbol").unwrap();
    let no_interactive = matches.get_flag("no-interactive");
    let tmux_mode = matches.get_flag("tmux");

    // determine editor
    let editor_arg = matches.get_one::<String>("editor");
    let editor_env = std::env::var("EDITOR").ok();

    let editor_owned = editor_arg
        .map(|s| s.clone())
        .or(editor_env)
        .unwrap_or_else(|| "vim".to_string());

    let editor = editor_owned.as_str();

    // search
    let results = search_symbol(path, symbol);

    if results.is_empty() {
        println!("No matches found for '{}'", symbol);
        return;
    }

    // determine which match to open
    let chosen = if no_interactive {
        &results[0]
    } else {
        match ui::select_match(&results, symbol) {
            Some(m) => m,
            None => {
                return;
            }
        }
    };

    open_in_editor(chosen, tmux_mode, editor);
}

