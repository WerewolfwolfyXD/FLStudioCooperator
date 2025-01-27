use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use base64::{encode};

pub fn dump(data: String) -> String {
    let now = SystemTime::now();
    return match now.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            let mut file = File::create(format!("./{}.bin", seconds)).unwrap();
            file.write_all(data.as_bytes()).unwrap();
            seconds.to_string()
        }
        Err(_) => { "data".to_string() }
    }
}

pub fn base64encode(data: String) -> String {
    return encode(data)
}