use rand::Rng;
use sys_info::{self};

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}
impl <S> Signal<S> where S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

#[derive(Clone)]
pub struct RandomSignal {
    distribution: u64,
    rng: u64,
}

impl RandomSignal {
    pub fn new() -> RandomSignal {
        RandomSignal {
            distribution: 50,
            rng: 50
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution + 1)
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub struct MemSignal {
    x: f64,
    free: f64,
}

impl MemSignal {
    pub fn new(free: f64) -> MemSignal {
        MemSignal {
            x: 0.0,
            free,
        }
    }
}

impl Iterator for MemSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        // let mut rng = rand::thread_rng();
        // let point = (self.x, rng.gen_range(1.0..3.4));
        let point = (self.x, self.free);
        self.x += 1.0;
        Some(point)
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub mem_signal: Signal<MemSignal>,
}

// impl Signals {
//     fn on_tick(&mut self) {
//         self.sin1.on_tick();
//     }
// }

pub struct App {
    pub num: i32,
    pub should_quit: bool,
    sparkline: Signal<RandomSignal>,
    pub signals: Signals,
    pub gas_info: GasInfo,
}

impl App {
    pub fn new() -> App {
        let gas_info = GasInfo::new();

        let mut rand_signal = RandomSignal::new();
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let sparkline = Signal {
            source: rand_signal,
            points: sparkline_points,
            tick_rate: 1,
        };

        let mut sin_signal = SinSignal::new(0.9, 1.0, 2.3);
        let sin1_points: Vec<(f64, f64)> = sin_signal.by_ref().take(100).collect();

        let free = (gas_info.mem_info.avail / 1024000) as f64;
        let mem_signal = MemSignal::new(free);
        // let mem_signal_points = mem_signal.by_ref().take(10).collect();
        let mem_signal_points: Vec<(f64, f64)>  = vec![(0.0, 0.0);1];
        let signals = Signals {
            sin1: Signal {
                source: sin_signal,
                points: sin1_points,
                tick_rate: 5
            },
            mem_signal: Signal { 
                source: mem_signal,
                points: mem_signal_points,
                tick_rate: 1
            }
        };
        App { num: 0, should_quit: false, sparkline, signals, gas_info: { GasInfo::new() } }
    }

    pub fn on_tick(&mut self) {
        self.sparkline.on_tick();
        self.signals.mem_signal.on_tick();
        self.gas_info.on_tick();
        self.num += 1;
    }
}

pub struct GasInfo {
    pub hostname: String,
    pub cpu_num: u32,
    pub disk_info: sys_info::DiskInfo,
    pub mem_info: sys_info::MemInfo,
}

impl Default for GasInfo {
    fn default() -> GasInfo {
        GasInfo {
            hostname: "".to_string(),
            cpu_num: 0,
            disk_info: sys_info::DiskInfo { total: 0, free: 0 },
            mem_info: sys_info::MemInfo { total: 0, free: 0, avail: (0), buffers: (0), cached: (0), swap_total: (0), swap_free: (0) }
        }
    }
}

impl GasInfo {
    pub fn on_tick (&mut self) {
        let new_gas_info = GasInfo::new();
        self.disk_info = new_gas_info.disk_info;
        self.mem_info = new_gas_info.mem_info;
    }

    pub fn new() -> GasInfo {
        let mut gas_info = GasInfo {..Default::default()};

        let cpu_num = sys_info::cpu_num();
        match cpu_num {
            Ok(c) => {
                gas_info.cpu_num = c;
            },
            Err(x) => {
                println!("error: {}", x)
            }
        }
    
        let disk_info = sys_info::disk_info();
        match disk_info {
            Ok(d) => {
                gas_info.disk_info = d;
            },
            Err(x) => {
                println!("error: {}", x)
            }
        }
    
        let hostname = sys_info::hostname();
        match hostname {
            Ok(h) => {
                gas_info.hostname = h;
            },
            Err(x) => {
                println!("error: {}", x)
            }
        }
    
        let mem_info = sys_info::mem_info();
        match mem_info {
            Ok(m) => {
                gas_info.mem_info = m;
            },
            Err(x) => {
                println!("error: {}", x)
            }
        }
    
        return gas_info;
    }
}
