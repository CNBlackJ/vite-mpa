mod app;
mod runner;
mod callback;
mod ui;

fn p() {
    let r = ui::run();
    match r {
        Ok(_) => {},
        Err(_) => {}
    };
}

fn main() {
    // runner::start(show);
    runner::start(p);
}
