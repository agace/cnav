use console::{style, Key, Term};
use crate::types::Match;

pub fn select_match<'a>(results: &'a [Match], symbol: &str) -> Option<&'a Match> {
    let term = Term::stdout();
    let mut index: usize = 0;
    let page_size: usize = 15;

    loop {
        term.clear_screen().unwrap();

        let total_pages = (results.len() + page_size - 1) / page_size;
        let current_page = index / page_size + 1;

        println!("Select a match to open (press 'q' to quit, k/j/h/l or just arrows keys to move, Enter to select)");
        println!("\n[PAGE {}/{}]", current_page, total_pages);

        let start = index.saturating_sub(index % page_size);
        let end = (start + page_size).min(results.len());

        for (i, m) in results[start..end].iter().enumerate() {
            let line = format!("{}:{} - {}", m.file, m.line, m.content.replace(
                symbol,
                &format!("{}", style(symbol).green().bold()),
            ));
            if start + i == index {
                println!(">> {}", style(line).on_bright().bold());
            } else {
                println!("  {}", line);
            }
        }

        match term.read_key().unwrap() {
            // single step
            Key::Char('k') | Key::ArrowUp => {
                if index > 0 {
                    index -= 1;
                }
            }

            Key::Char('j') | Key::ArrowDown => {
                if index + 1 < results.len() {
                    index += 1;
                }
            }

            // page navigation
            Key::Char('h') | Key::ArrowLeft => {
                if index >= page_size {
                    index -= page_size;
                } else {
                    index = 0;
                }
            }

            Key::Char('l') | Key::ArrowRight => {
                if index + page_size < results.len() {
                    index += page_size;
                } else {
                    index = results.len() -1;
                }
            }
            Key::Enter => return Some(&results[index]),
            Key::Char('q') => return None,
            _ => {}
        }
    }
}

