use sys_info::{self};

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

pub fn show_info() -> GasInfo {

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
