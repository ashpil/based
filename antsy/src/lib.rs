use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::process::Command;

pub struct Printer {
    stdout: Stdout,
    width: u16,
}

const ESC: &str = "\x1b[";
const _RESET: &str = "\x1b[0m";

impl Printer {
    pub fn new() -> Printer {
        let mut stdout = stdout();
        stdout.write(b"\n").unwrap();
        let width =  String::from_utf8_lossy(&Command::new("tput").arg("cols").output().unwrap().stdout).trim().parse::<u16>().unwrap();
        Printer {
            stdout: stdout,
            width,
        }
    }

    pub fn progress_bar(&mut self, percent_done: f64) {
        self.stdout.write((String::from(ESC) + "999D").as_bytes()).unwrap();
        self.stdout.write(b"[").unwrap();
        let num_done = ((self.width - 2) as f64 * percent_done).round() as usize;
        let num_todo = ((self.width - 2) as f64 * (1.0 - percent_done)).round() as usize;
        let done = "█".repeat(num_done);
        let todo = "░".repeat(num_todo);
        self.stdout.write(done.as_bytes()).unwrap();
        self.stdout.write(todo.as_bytes()).unwrap();
        self.stdout.write(b"]").unwrap();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut p = Printer::new();
        p.progress_bar(0.1);
        p.progress_bar(0.2);
    }
}
