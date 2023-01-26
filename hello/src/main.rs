enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

struct PCSpec {
    cpus: u16,
    memory: u32,
    storage: Storage,
}

fn main() {
    let spec = PCSpec {
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };

    match &spec {
        PCSpec {
            storage: Storage::SSD(512),
            ..
        } => {
            println!("512GiB SSD");
        }
        PCSpec {
            cpus: 4 | 8,
            memory: m,
            storage: _,
        } => {
            println!("4 or 8 CPUs");
            println!("{}GiB memory", *m);
        }
        PCSpec { memory: m, .. } if *m < 4 => {
            println!("less than 4GiB memory")
        }
        _ => (),
    }
}
