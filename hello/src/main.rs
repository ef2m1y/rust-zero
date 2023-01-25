// enum DoW {
//     Sunday,
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursay,
//     Friday,
//     Saturday,
// }

enum Storage {
    HDD { size: u32, rpm: u32 },
    SSD(u32),
}

fn main() {
    let hdd = Storage::HDD { size: 512, rpm: 7200 };
    let ssd = Storage::SSD(512);
}