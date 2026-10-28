const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GOLD: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";

pub fn color(code: &str, text: impl std::fmt::Display) -> String {
    format!("{code}{text}{RESET}")
}

pub fn hp_bar(current: u32, max: u32, width: usize) -> String {
    let filled = if max == 0 {
        0
    } else {
        (current as usize * width / max as usize).min(width)
    };
    format!("[{}{}]", "#".repeat(filled), ".".repeat(width - filled))
}

pub fn panel(title: &str, lines: &[String]) {
    let width = lines
        .iter()
        .map(String::len)
        .max()
        .unwrap_or(0)
        .max(title.len())
        + 4;
    println!(
        "┌─ {} {}┐",
        title,
        "─".repeat(width.saturating_sub(title.len() + 3))
    );
    for line in lines {
        println!("│ {:width$} │", line, width = width);
    }
    println!("└{}┘", "─".repeat(width + 2));
}

pub fn welcome() {
    println!(
        "{}",
        color(
            CYAN,
            "╔════════════════════════════════════════════════════════════╗"
        )
    );
    println!(
        "{}",
        color(
            CYAN,
            "║                 E C H O E S  O F  A S H                   ║"
        )
    );
    println!(
        "{}",
        color(
            CYAN,
            "║          A terminal adventure of choice and consequence   ║"
        )
    );
    println!(
        "{}",
        color(
            CYAN,
            "╚════════════════════════════════════════════════════════════╝"
        )
    );
}

pub fn legend() {
    println!(
        "{} location  {} healthy  {} reward  {} danger",
        color(CYAN, "●"),
        color(GREEN, "●"),
        color(GOLD, "●"),
        color(RED, "●")
    );
    println!(
        "{}",
        color(
            DIM,
            "Tip: type `help` for commands, `map` to see the realm."
        )
    );
}
