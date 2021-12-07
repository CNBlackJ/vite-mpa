use sys_info;

fn main() {
    sys_info::cpu_num();
    let a = sys_info::disk_info();
    println!("{:#?}", a);
}
