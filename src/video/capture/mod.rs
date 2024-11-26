pub mod parameters;
pub use parameters::Parameters;

use std::convert::TryFrom;
use std::{io, mem};

use crate::buffer::Type;
use crate::device::Device;
use crate::format::{FormatMplane, FourCC};
use crate::format::{Description as FormatDescription, Format};
use crate::frameinterval::FrameInterval;
use crate::framesize::FrameSize;
use crate::v4l2;
use crate::v4l_sys::*;
use crate::video::traits::{Capture, CaptureMplane};


macro_rules! impl_params {
    ($typ:expr) => {
        fn params(&self) -> io::Result<Parameters> {
            unsafe {
                let mut v4l2_params = v4l2_streamparm {
                    type_: $typ as u32,
                    ..mem::zeroed()
                };
                v4l2::ioctl(
                    self.handle().fd(),
                    v4l2::vidioc::VIDIOC_G_PARM,
                    &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
                )?;

                Ok(Parameters::from(v4l2_params.parm.capture))
            }
        }
    }
}

macro_rules! impl_set_params {
    ($typ:expr, $device:ident) => {
        fn set_params(&self, params: &Parameters) -> io::Result<Parameters> {
            unsafe {
                let mut v4l2_params = v4l2_streamparm {
                    type_: $typ as u32,
                    parm: v4l2_streamparm__bindgen_ty_1 {
                        capture: (*params).into(),
                    },
                };
                v4l2::ioctl(
                    self.handle().fd(),
                    v4l2::vidioc::VIDIOC_S_PARM,
                    &mut v4l2_params as *mut _ as *mut std::os::raw::c_void,
                )?;
            }

            $device::params(self)
        }
    }
}

impl Capture for Device {
    impl_enum_frameintervals!();
    impl_enum_framesizes!();
    impl_enum_formats!(Type::VideoCapture);
    impl_format!(Type::VideoCapture, pix, Format);
    impl_set_format!(Type::VideoCapture, pix, Format, Capture);
    impl_params!(Type::VideoCapture);
    impl_set_params!(Type::VideoCapture, Capture);
}

impl CaptureMplane for Device {
    impl_enum_frameintervals!();
    impl_enum_framesizes!();
    impl_enum_formats!(Type::VideoCaptureMplane);
    impl_format!(Type::VideoCaptureMplane, pix_mp, FormatMplane);
    impl_set_format!(Type::VideoCaptureMplane, pix_mp, FormatMplane, CaptureMplane);
    impl_params!(Type::VideoCaptureMplane);
    impl_set_params!(Type::VideoCaptureMplane, CaptureMplane);
}
