use chrono::Local;

pub fn get_current_time() -> String {
    Local::now().format("%H:%M:%S").to_string()
}