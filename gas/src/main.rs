use core::time::Duration;
use std::time::Instant;
use app::App;

mod app;
mod ui;

fn main() {
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    let mut app_ins = App::new();

    loop {
        ui::run(&mut app_ins).unwrap();

        if last_tick.elapsed() >= tick_rate {
            app_ins.on_tick();
            last_tick = Instant::now();
        }

        if app_ins.num > 50 {
            return
        }
    }
}
