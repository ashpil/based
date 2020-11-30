use std::io::stdout;
use std::io::Stdout;
use std::io::Write;
use std::process::Command;
use std::time::{Instant, Duration};

const ESC: &str = "\x1b[";
const RESET: &str = "\x1b[0m";


pub struct LoadingBar {
    max: u16,
    start_time: Instant,
    width: u16,
    stdout: Stdout,
}

impl LoadingBar {
    pub fn new(max: u16) -> LoadingBar {
        let mut stdout = stdout();
        stdout.write(b"\n").unwrap();
        stdout.write((String::from(ESC) + "s").as_bytes()).unwrap();
        stdout.write(b"\n").unwrap();
        let width =  String::from_utf8_lossy(&Command::new("tput").arg("cols").output().unwrap().stdout).trim().parse::<u16>().unwrap();
        let start_time = Instant::now();
        LoadingBar {
            stdout,
            width,
            start_time,
            max,
        }
    }

    // Updates time, bar, and count
    pub fn update(&mut self, num_done: u16) {
        self.stdout.write((String::from(ESC) + "u").as_bytes()).unwrap();
        self.stdout.write((String::from(ESC) + "1A").as_bytes()).unwrap();
        let done = format!("{:>1$}", num_done, self.max.to_string().len());
        let count = format!(" {}/{} ", done, self.max);
        let time = format_duration(self.start_time.elapsed());
        self.stdout.write(time.as_bytes()).unwrap();
        self.stdout.write(b"[").unwrap();
        let reserved_len = (count.len() + time.len()) as u16 + 2;
        let percent_done = num_done as f64 / self.max as f64;
        let num_done = ((self.width - reserved_len) as f64 * percent_done).round() as usize;
        let num_todo = ((self.width - reserved_len) as f64 * (1.0 - percent_done)).round() as usize;
        let bar = format!("{}{}",
                          make_color(" ".repeat(num_done), 104),
                          make_color(" ".repeat(num_todo), 107));
        self.stdout.write(bar.as_bytes()).unwrap();
        self.stdout.write(b"]").unwrap();
        self.stdout.write(count.as_bytes()).unwrap();
        self.stdout.write(b"\n").unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    // Finishes bar; writes total time taken
    pub fn finish(&mut self) {
        let time_str = format!("Took {} seconds", self.start_time.elapsed().as_secs_f64());
        let finish_str = format!("{:^1$}", time_str, self.width as usize);
        self.update(self.max);
        self.stdout.write(finish_str.as_bytes()).unwrap();
        self.stdout.write(b"\n").unwrap();
    }
}

// Pretty prints a duration in the form of [hh:mm:ss]
fn format_duration(duration: Duration) -> String {
    let time = duration.as_secs();
    let seconds = time % 60;
    let minutes = (time / 60) % 60;
    let hours = time / 3600;
    let seconds_str = format!("{:0>1$}", seconds, 2);
    let minutes_str = format!("{:0>1$}", minutes, 2);
    let hours_str = format!("{:0>1$}", hours, 2);
    format!(" [{}:{}:{}] ", hours_str, minutes_str, seconds_str)
}

// Prints something a color given by an ANSI escape code number
fn make_color(s: String, color: u8) -> String {
    format!("{}{}m{}{}", ESC, color, s, RESET) 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    // Just a visual test for now, so I can confirm it works
    #[test]
    fn bar() {
        let mut b = LoadingBar::new(300);
        b.update(10);
        b.flush();
        let second = Duration::from_secs(1);
        thread::sleep(second);
        b.update(20);
        b.flush();
        thread::sleep(second);
        b.update(30);
        b.flush();
        thread::sleep(second);
        b.finish();
    }
}
