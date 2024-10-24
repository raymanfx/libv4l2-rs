//! This crate provides safe bindings for the Video4Linux (v4l) stack.
//!
//! The stack consists of three libraries written in C:
//! * libv4l1       (v4l1 API, deprecated)
//! * libv4l2       (v4l2 API, the primary target of this crate)
//! * libv4lconvert (emulates common formats such as RGB3 in userspace)
//!
//! Additional documentation can currently also be found in the
//! [README.md file which is most easily viewed on github](https://github.com/raymanfx/libv4l-rs/blob/master/README.md).
//!
//! [Jump forward to crate content](#reexports)
//!
//! # Overview
//!
//! Video devices on Linux can be accessed by path or by index (which then corresponds to a path),
//! e.g. "/dev/video0" for the device which first became known to the system.
//!
//! There are three methods of dealing with (capture) device memory:
//! * `MMAP` (memory region in device memory or kernel space, mapped into userspace)
//! * `User` pointer (memory region allocated in host memory, written into by the kernel)
//! * `DMA` (direct memory access for memory transfer without involving the CPU)
//!
//! The following schematic shows the `mmap` and `userptr` mechanisms:
//!
//! **mmap**
//!
//! 1. `device --[MAP]--> kernel --[MAP]--> user`
//! 2. `device --[DMA]--> kernel --[MAP]--> user`
//!
//! **userptr**
//!
//! 3. `device --[DMA]-->                   user`
//!
//!
//! It is important to note that user pointer is for device-to-user memory transfer whereas
//! DMA is for device-to-device transfer, e.g. directly uploading a captured frame into GPU
//! memory.
//!
//! As you can see, user pointer and DMA are potential candidates for zero-copy applications where
//! buffers should be writable. If a read-only buffer is good enough, MMAP buffers are fine and
//! do not incur any copy overhead either. Most (if not all) devices reporting streaming I/O
//! capabilities support MMAP buffer sharing, but not all support user pointer access.
//!
//! The regular user of this crate will mainly be interested in frame capturing.
//! Here is a very brief example of streaming I/O with memory mapped buffers:
//!
//! ```no_run
//! use v4l::buffer::Type;
//! use v4l::io::traits::CaptureStream;
//! use v4l::prelude::*;
//!
//! let mut dev = Device::new(0).expect("Failed to open device");
//!
//! let mut stream =
//!     MmapStream::with_buffers(&mut dev, Type::VideoCapture, 4).expect("Failed to create buffer stream");
//!
//! loop {
//!     let (buf, meta) = stream.next().unwrap();
//!     println!(
//!         "Buffer size: {}, seq: {}, timestamp: {}",
//!        buf.len(),
//!        meta.sequence,
//!        meta.timestamp
//!    );
//!}
//!```
//!
//! Have a look at the examples to learn more about device and buffer management.

#[cfg(feature = "v4l-sys")]
pub use v4l_sys;

#[cfg(feature = "v4l2-sys")]
pub use v4l2_sys as v4l_sys;

pub mod v4l2;

pub mod buffer;
pub mod capability;
pub mod context;
pub mod control;
pub mod device;
pub mod format;
pub mod fraction;
pub mod frameinterval;
pub mod framesize;
pub mod memory;
pub mod parameters;
pub mod timestamp;
pub mod video;

pub mod io;

pub use {
    capability::Capabilities,
    control::Control,
    device::Device,
    format::{Format, FourCC},
    fraction::Fraction,
    frameinterval::FrameInterval,
    framesize::FrameSize,
    memory::Memory,
    timestamp::Timestamp,
};

pub mod prelude {
    pub use crate::device::Device;
    pub use crate::io::{dmabuf::Stream as DmabufStream, mmap::Stream as MmapStream, userptr::Stream as UserptrStream};
}
