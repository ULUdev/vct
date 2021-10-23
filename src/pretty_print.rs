use crate::Vocab;
use btui::effects::{Color, Special};
use btui::print::{fg, sp};

pub fn pretty_print(vocab: Vec<Vocab>) -> String {
    let mut out: String = String::new();

    for voc in vocab {
        let meanings: String = voc
            .get_meanings()
            .iter()
            .map(|x| format!("  {}- {}{}\n", fg(Color::Blue), x, sp(Special::Reset)))
            .collect();
        out.push_str(format!("\n{}{}:\n{}", fg(Color::Green), voc.get_name(), meanings).as_str());
        if let Some(n) = voc.get_additionals() {
            let adds: String = n
                .iter()
                .map(|x| {
                    let mut parts = x.split(':');
                    let key: &str = match parts.next() {
                        Some(n) => n,
                        None => {
                            return String::new();
                        }
                    };
                    let value: &str = match parts.next() {
                        Some(n) => n,
                        None => {
                            return String::new();
                        }
                    };
                    format!(
                        "  {}{}: {}{}\n",
                        fg(Color::Yellow),
                        key,
                        value,
                        sp(Special::Reset)
                    )
                })
                .collect();
            out.push_str(adds.as_str());
        }
    }

    out
}
