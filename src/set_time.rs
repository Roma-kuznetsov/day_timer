use std::fs::{File, OpenOptions};
use std::io::{self, Write};

use std::thread;
use std::time::Duration;

pub fn set_time(filename: &str, file_day: &str, file_time: &str) -> io::Result<()> {
    let number = file_time.parse::<u32>().unwrap();

    thread::sleep(Duration::new(60, 0));
    let res = number + 1;

    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true) // Очищаем файл перед записью
        .open(filename)?;

    let string = format!("{file_day}-{res}");
    writeln!(file, "{string}")?;

    Ok(())
}
