fn main() {
    let buf = Buffer::<128> { buf: [0; 128] };
}

struct Buffer<const S: usize> {
    buf: [u8; S],
}
