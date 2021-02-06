use std::io::{Stdout, Write, self};
use std::process::Command;
use std::time::{Instant, Duration};

const ESC: &str = "\x1b[";
const RESET: &str = "\x1b[0m";

pub struct LoadingBar {
    max: u32,
    start_time: Instant,
    width: u32,
    num_done: u32,
    advance_every: u32,
    stdout: Stdout,
}

impl LoadingBar {
    pub fn new(max: u32, advance_every: u32) -> Result<LoadingBar, io::Error> {
        let std = io::stdout();
        let mut handle = std.lock();
        writeln!(handle,)?;
        writeln!(handle, "{}", (String::from(ESC) + "s"))?;
        let width =  String::from_utf8_lossy(&Command::new("tput").arg("cols").output().unwrap().stdout).trim().parse::<u32>().unwrap();
        let start_time = Instant::now();
        Ok(LoadingBar {
            stdout: io::stdout(),
            width,
            start_time,
            max,
            advance_every,
            num_done: 0,
        })
    }

    // Advances time, bar, and count
    pub fn advance(&mut self) -> io::Result<()> {
        let mut handle = self.stdout.lock();
        let num_done = self.num_done / self.advance_every;
        write!(handle, "{}u", ESC)?;
        write!(handle, "{}1A", ESC)?;
        let time = format_duration(self.start_time.elapsed());
        write!(handle, "{}", time)?;
        let done = format!("{:>1$}", num_done, ((self.max as f64).log10() + 1.0).floor() as usize);
        let count = format!(" {}/{} ", done, self.max);
        let reserved_len = (count.len() + time.len()) as u32 + 2;
        let percent_done = num_done as f64 / self.max as f64;
        let num_done = ((self.width - reserved_len) as f64 * percent_done).round() as usize;
        let num_todo = ((self.width - reserved_len) as f64 * (1.0 - percent_done)).round() as usize;
        write!(handle, "[{}{}]", 
                          make_color(" ".repeat(num_done), 104),
                          make_color(" ".repeat(num_todo), 107))?;
        self.num_done += 1;
        writeln!(handle, "{}", count)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub fn get_elapsed(&self) -> Duration {
        self.start_time.elapsed()
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

