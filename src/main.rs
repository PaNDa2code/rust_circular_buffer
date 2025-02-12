use std::io::Read;
use std::io::Write;

mod ring_buffer;

fn main() {
    let mut ring_buffer = ring_buffer::RingBuffer::new(64 * 1024).unwrap();

    for _ in 0..12 * 1024 {
        _ = ring_buffer.write(b"0123456789ABCDEF");
    }

    let string_slice = unsafe { std::str::from_utf8_unchecked(ring_buffer.to_slice()) };
    println!("{:?}", ring_buffer);
    println!("{}", ring_buffer.len());
    println!(
        "{}...{}",
        &string_slice[..10],
        &string_slice[64 * 1024 - 10..]
    );
}
