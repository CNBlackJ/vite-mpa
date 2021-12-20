type Callback = fn();

pub struct Processor {
    pub callback: Callback,
}

impl Processor {
    // fn set_callback(&mut self, c: Callback) {
    //     self.callback = c;
    // }

    pub fn process_events(&self) {
        (self.callback)();
    }
}
