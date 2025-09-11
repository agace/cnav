use std::process::Command;
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

pub fn open_in_editor(chosen: &Match, use_tmux: bool, editor: &str) {
    let (program, args) = build_editor_command(editor, &chosen.file, chosen.line);

    if use_tmux && std::env::var("TMUX").is_ok() {
        Command::new("tmux")
            .arg("split-window")
            .arg("-h")
            .arg(&program)
            .args(&args)
            .status()
            .unwrap_or_else(|e| panic!("Failed to launch tmux split with {}: {}", editor, e));
    } else {
        Command::new(&program)
            .args(&args)
            .status()
            .unwrap_or_else(|e| panic!("Failed to launch {}: {}", editor, e));
    }
}

