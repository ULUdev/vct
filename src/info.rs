use btui::effects::{Color, Special};
use btui::print::{fg, sp};
use btui::Terminal;

/// enum representing different types of messages to be logged to stderr
pub enum MessageType {
    Warning,
    Error,
    // Log,
}

/// print a message to stderr formatted according to the programs style
/// # Arguments
/// * `term`: a reference to a btui terminal (used for printing the message)
/// * `msg`: the message to print (can be any type that implements `std::fmt::Display`)
/// * `msg_type`: the type of the message
///
/// # Panics
/// If any of the printing fails this function will panic since vct requires printing functionality
/// to be in place
pub fn print_info<T: std::fmt::Display>(term: &Terminal, msg: T, msg_type: MessageType) {
    match msg_type {
        MessageType::Warning => {
            term.eprintln(format!(
                "{}vct: warning: {}{}",
                fg(Color::Yellow),
                msg,
                sp(Special::Reset)
            ))
            .unwrap();
        }
        MessageType::Error => {
            term.eprintln(format!(
                "{}vct: error: {}{}",
                fg(Color::Red),
                msg,
                sp(Special::Reset)
            ))
            .unwrap();
        } // MessageType::Log => {
          //     term.eprintln(format!("{}vct: log: {}{}", fg(Color::White), msg, sp(Special::Reset))).unwrap();
          // }
    }
}
