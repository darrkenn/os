use crate::time::datetime::DateTime;

pub fn from_secs(seconds: u32) {
    if seconds > 60 {
        return;
    }
    let mut end_time: DateTime = DateTime::new();
    end_time.add_seconds(seconds);
    loop {
        let now = DateTime::new();
        if now >= end_time {
            break;
        }
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
}

