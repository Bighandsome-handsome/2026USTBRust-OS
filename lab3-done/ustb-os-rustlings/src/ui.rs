use console::style;

macro_rules! warn {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("!").red(), style(formatstr).red());
        } else {
            println!(
                "{} {}",
                style(Emoji("⚠️ ", "!")).red(),
                style(formatstr).red()
            );
        }
    }};
}

macro_rules! success {
    ($fmt:literal, $ex:expr) => {{
        use console::{style, Emoji};
        use std::env;
        let formatstr = format!($fmt, $ex);
        if env::var("NO_EMOJI").is_ok() {
            println!("{} {}", style("✓").green(), style(formatstr).green());
        } else {
            println!(
                "{} {}",
                style(Emoji("✅", "✓")).green(),
                style(formatstr).green()
            );
        }
    }};
}

/// Clears the terminal with an ANSI escape code.
/// Works in UNIX and newer Windows terminals.
pub fn clear_screen() {
    println!("\x1Bc");
}

pub fn show_interactive_prompt() {
    println!();
    print!("{}:hint / {}: help / {}:quit ? ",
        style("h").bold(),
        style("e").bold(),
        style("q").bold(),
    );
    use std::io::{self, Write};
    io::stdout().flush().unwrap();
}