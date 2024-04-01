use std::io;
use std::time::Instant;

use v4l::buffer::Type;
use v4l::capability::Flags;
use v4l::format::FieldOrder;
use v4l::{prelude::*, Format, FourCC};
use v4l::io::mmap::MPlaneStream;
use v4l::video::MultiPlanarCapture;
use v4l::io::traits::CaptureStream; 

fn main() -> io::Result<()> {
    let path = "/dev/video22";
    println!("Using device: {}\n", path);

    let dev = Device::with_path(path)?;

    let caps = dev.query_caps()?;
    if !caps.capabilities.contains(Flags::VIDEO_CAPTURE_MPLANE) {
        println!("{path} is no Video capture mplane device");
        return Err(io::Error::last_os_error());
    }

    if !caps.capabilities.contains(Flags::STREAMING) {
        println!("{path} does not support streaming i/o");
        return Err(io::Error::last_os_error());
    }

    let mut format = Format::new(640, 480, FourCC::new(b"NV12"));
    format.field_order = FieldOrder::Interlaced;
    dev.set_format(&format)?;

    let count = 4;

    let mut mplane_stream = MPlaneStream::new(&dev, Type::VideoCaptureMplane, 1)?;

    mplane_stream.next()?;

    // let start = Instant::now();
    // let mut megabytes_ps: f64 = 0.0;
    // for i in 0..count {
    //     let t0 = Instant::now();
    //     let (buf, meta) = mplane_stream.next()?;
    //     let duration_us = t0.elapsed().as_micros();

    //     let cur = buf.len() as f64 / 1_048_576.0 * 1_000_000.0 / duration_us as f64;
    //     if i == 0 {
    //         megabytes_ps = cur;
    //     } else {
    //         // ignore the first measurement
    //         let prev = megabytes_ps * (i as f64 / (i + 1) as f64);
    //         let now = cur * (1.0 / (i + 1) as f64);
    //         megabytes_ps = prev + now;
    //     }

    //     println!("Buffer");
    //     println!("  sequence  : {}", meta.sequence);
    //     println!("  timestamp : {}", meta.timestamp);
    //     println!("  flags     : {}", meta.flags);
    //     println!("  length    : {}", buf.len());
    // }    

    // println!();
    // println!("FPS: {}", count as f64 / start.elapsed().as_secs_f64());
    // println!("MB/s: {}", megabytes_ps);

    Ok(())
}