use chrono::{DateTime, Datelike, Local};
use winapi::um::wincon::FreeConsole;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

mod bash_script;
mod set_time;

fn main() -> io::Result<()> {
    // скрываем консоль
    unsafe {
        FreeConsole();
    }

    loop {
        // Пути к фалам файла
        let file_timer: &str = "current_day.txt";
        let file_config: &str = "config.txt";
        // Получаем текущее локальное время
        let now: DateTime<Local> = Local::now();
        // Извлекаем текущий день
        let day: u32 = now.day();
        // Формат дня вида "0 + <number>"
        let formatted_day = format!("{:02}", day);
        // обнуление таймера
        let start_time: &str = "000";
        // информация дня из файла
        let file_day: &str;
        // информация времени из файла
        let file_time: &str;
        // ограничение по времени из конфига
        let config_time: &str;
        // время из конфига в виде u32
        let mut config_time_number: u32 = 150;

        //чтение конфига
        if Path::new(file_config).exists() {
            // Если файл существует, открываем его для чтения
            let mut file: File = File::open(file_config)?;
            let mut file_config_limit: String = String::new();
            file.read_to_string(&mut file_config_limit)?; // достаем данные из файла config.txt
            config_time = &file_config_limit.trim()[..];
            config_time_number = config_time.parse::<u32>().unwrap();
        }

        // Проверяем, существует ли файл
        if Path::new(file_timer).exists() {
            // Если файл существует, открываем его для чтения
            let mut file: File = File::open(file_timer)?;
            let mut contents_timer: String = String::new();
            file.read_to_string(&mut contents_timer)?; // достаем данные из файла current_day.txt

            file_day = &contents_timer.trim()[..2];
            file_time = &contents_timer.trim()[3..];
            println!("{file_time}");
            // Проверяем, совпадает ли день из файла с текущим днем
            if file_day == &format!("{}", formatted_day) {
                let check: u32 = file_time.parse::<u32>().unwrap();
                if check >= config_time_number {
                    let _ = bash_script::bash_script(now);
                }
                let _ = set_time::set_time(file_timer, file_day, file_time);
                continue;
            }
        }

        // Если файл не существует или дни не совпадают, открываем файл для записи
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true) // Очищаем файл перед записью
            .open(file_timer)?;

        // Записываем текущий день в файл

        let string = format!("{formatted_day}-{start_time}");
        writeln!(file, "{string}")?;
    }
}
