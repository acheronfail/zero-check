use std::env::args;
use std::fs::File;
use std::io::Read;

const BUF_SIZE: usize = 2_000_000;
fn main() {
    let path = &args().collect::<Vec<_>>()[1];

    let mut buf = vec![0; BUF_SIZE];
    let mut file = File::open(&path).unwrap();

    let mut zero_bytes = 0f64;
    let mut total_bytes = 0f64;
    loop {
        let len = file.read(&mut buf).unwrap();
        if len == 0 {
            break;
        }

        for i in 0..len {
            total_bytes += 1.0;
            if buf[i] == 0 {
                zero_bytes += 1.0;
            }
        }
    }

    println!("{: >6.2}% --- {}", (zero_bytes / total_bytes) * 100.0, path);
}
