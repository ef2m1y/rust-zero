enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

struct PCSpec {
    cpus: u16,
    memory: u32,
    storage: Storage,
}

struct Dim2(u32, u32);

fn main() {
    let spec = PCSpec {
        cpus: 8,
        memory: 16,
        storage: Storage::SSD(1024),
    };
    println!("{}", spec.cpus);

    let d2 = Dim2(10, 20);
    println!("{}", d2.0);

    let r = &spec;
    println!("{}", (*r).cpus);
    println!("{}", r.cpus); // auto dereference
}
