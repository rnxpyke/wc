use std::{fs::File, io::Read};
use wc::*;

#[cfg(feature = "uring")]
fn io_uring(file: std::fs::File) -> WC {
    const BUF: usize = 4096 * 512;
    tokio_uring::start(async {
        let mut wc = WC::default();
        let file = tokio_uring::fs::File::from_std(file);
        let (res, mut blue) = file.read_at(vec![0u8; BUF], 0).await;
        let mut offset = res.unwrap();
        let mut bytes = offset;
        let mut red = vec![0u8; BUF];
        loop {
            let readfut = file.read_at(red, offset as u64);
            wc.write(&blue[..bytes]).unwrap();
            let (res, buf) = readfut.await;
            red = buf;
            bytes = res.unwrap();
            if bytes == 0 {
                break;
            }
            offset += bytes;

            std::mem::swap(&mut blue, &mut red);
        }
        file.close().await.unwrap();
        wc
    })
}

#[cfg(not(feature = "uring"))]
fn sync(mut file: std::fs::File) -> WC {
    use std::io::Write;

    const BUF: usize = 4096 * 512;
    let mut buf = vec![0u8; BUF];
    let mut wc = WC::default();
    loop {
        let bytes = file.read(&mut buf).unwrap();
        wc.write(&buf[..bytes]).unwrap();
        if bytes == 0 {
            break;
        }
    }
    wc
}

#[cfg(not(feature = "uring"))]
fn count_file(file: std::fs::File) -> WC {
    sync(file)
}

#[cfg(feature = "uring")]
fn count_file(file: std::fs::File) -> WC {
    io_uring(file)
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let path = args.next().expect("no file arg");
    let file = File::open(path).expect("file not found");
    let wc = count_file(file);
    println!("{}", wc.newlines);
}
