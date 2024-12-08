use std::process::{exit, Command};
use std::thread;
use std::time::Duration;
pub fn bash_script() {
    // Пример команды Bash, такую как 'ls' для Linux или 'dir' для Windows

    let output = Command::new("shutdown")
        .args(&["/f", "/s", "/t", "0"]) // немедленное завершение
        .output()
        .expect("Не удалось запустить процесс");

    if output.status.success() {
        println!("Команда успешно выполнена.");
    } else {
        let exit_code = output.status.code().unwrap_or(-1);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "Ошибка при выполнении команды (код выхода: {}):\n{}",
            exit_code, stderr
        );
        exit(exit_code);
    }
    thread::sleep(Duration::new(2, 0));
}
