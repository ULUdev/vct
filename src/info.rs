use btui::effects::{Color, Special};
use btui::print::{fg, sp};
use btui::Terminal;

pub enum MessageType {
    Warning,
    Error,
    // Log,
}

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
