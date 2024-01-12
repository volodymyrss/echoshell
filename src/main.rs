use std::thread::sleep;
use std::time::Duration;

fn main() {
    loop {
        println!("sleeping... Sending command: sleep 60");
        sleep(Duration::from_secs(5));
    }
}
