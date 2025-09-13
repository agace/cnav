use std::process::Command;
use std::path::Path;
use crate::types::Match;

fn build_editor_command(editor: &str, file: &str, line: usize) -> (String, Vec<String>) {
    match editor {
        "vim" | "nvim" | "emacs" | "kak" | "nano" | "joe" => {
            (editor.to_string(), vec![format!("+{}", line), file.to_string()])
        }
        "micro" => {
            (editor.to_string(), vec![format!("{}:+{}", file, line)])
        }
        "hx" => {
            (editor.to_string(), vec![format!("{}:{}", file, line)])
        }
        _ => {
            // fallback: open without line number
            (editor.to_string(), vec![file.to_string()])
        }
    }
}

pub fn command_exists(command: &str) -> bool {

    if command.contains('/') {
        return Path::new(command).exists();
    }
    
    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            let full_path = Path::new(dir).join(command);
            if full_path.exists() {
                return true;
            }
        }
    }
    
    false
}

pub fn open_in_editor(chosen: &Match, use_tmux: bool, editor: &str) {
    let (program, args) = build_editor_command(editor, &chosen.file, chosen.line);

    if use_tmux {
        Command::new("tmux")
            .arg("split-window")
            .arg("-h")
            .arg(&program)
            .args(&args)
            .status()
            .expect("Failed to open in tmux split");
    } else {
        Command::new(&program)
            .args(&args)
            .status()
            .expect("Failed to launch editor");
    }
}
