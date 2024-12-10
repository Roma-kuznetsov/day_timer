use chrono::{DateTime, Local};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::{exit, Command};

pub fn bash_script(now: DateTime<Local>) -> io::Result<()> {

    let output = Command::new("shutdown")
        .args(&["/f", "/s", "/t", "30"]) 
        .output()
        .expect("Не удалось запустить процесс");

    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open("log.txt")?;

    if output.status.success() {
        writeln!(file, "Всё должно быть заебись {now}")?;
        exit(10);
    } else {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        writeln!(
            file,
            "Ошибка при выполнении команды (код выхода: {exit_code}):\n{stderr} :\n {now}"
        )?;
        exit(exit_code);
    }
}
