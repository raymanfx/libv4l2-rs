use std::io;
use std::time::Instant;

use v4l::buffer::Type;
use v4l::io::traits::CaptureStream;
use v4l::prelude::*;
use v4l::video::Capture;

fn main() -> io::Result<()> {
    let path = "/dev/video0";
    println!("Using device: {}\n", path);

    // Capture 4 frames by default
    let count = 4;

    // Allocate 4 buffers by default
    let buffer_count = 4;

    let dev = Device::with_path(path)?;
    let format = dev.format()?;
    let params = dev.params()?;
    println!("Active format:\n{}", format);
    println!("Active parameters:\n{}", params);

    // Setup a buffer stream and grab a frame, then print its data
    let mut stream = DmabufStream::with_buffers(&dev, Type::VideoCapture, buffer_count)?;

    // warmup
    stream.next()?;

    let start = Instant::now();
    let mut megabytes_ps: f64 = 0.0;
    for i in 0..count {
        let t0 = Instant::now();
        let (buf, meta) = stream.next()?;
        let duration_us = t0.elapsed().as_micros();

        println!("Buffer");
        println!("  sequence  : {}", meta.sequence);
        println!("  timestamp : {}", meta.timestamp);
        println!("  flags     : {}", meta.flags);
        use std::fs::File;
        use std::os::fd::{FromRawFd, AsRawFd, IntoRawFd};
        use memmap2;
        let outf = unsafe { File::from_raw_fd(buf.as_raw_fd()) };
        let outfmap = unsafe { memmap2::Mmap::map(&outf) }?;
        println!("  length    : {}", outfmap.len());
        
        let cur = outfmap.len() as f64 / 1_048_576.0 * 1_000_000.0 / duration_us as f64;
        if i == 0 {
            megabytes_ps = cur;
        } else {
            // ignore the first measurement
            let prev = megabytes_ps * (i as f64 / (i + 1) as f64);
            let now = cur * (1.0 / (i + 1) as f64);
            megabytes_ps = prev + now;
        }
        // Prevent File from dropping and closing the fd, because fd is borrowed from texture and not ours to close.
        let _ = File::into_raw_fd(outf);
    }

    println!();
    println!("FPS: {}", count as f64 / start.elapsed().as_secs_f64());
    println!("MB/s: {}", megabytes_ps);

    Ok(())
}
