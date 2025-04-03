use std::thread;
use std::time::Duration;

pub fn initialize_gui() {
    thread::spawn(move || {
        loop {
            render_gui();
            thread::sleep(Duration::from_millis(100));
        }
    });
}

fn render_gui() {
    println!("Rendering GUI...");
}