use std::{convert::TryFrom, fmt, mem};

use crate::v4l_sys::*;

pub mod colorspace;
pub use colorspace::Colorspace;

pub mod description;
pub use description::Description;

pub mod field;
pub use field::FieldOrder;

pub mod fourcc;
pub use fourcc::FourCC;

pub mod quantization;
pub use quantization::Quantization;

pub mod transfer;
pub use transfer::TransferFunction;

bitflags::bitflags! {
    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
    pub struct Flags : u32 {
        const PREMUL_ALPHA  = 0x00000001;
    }
}

impl From<u32> for Flags {
    fn from(flags: u32) -> Self {
        Self::from_bits_retain(flags)
    }
}

impl From<u8> for Flags {
    fn from(flags: u8) -> Self {
        Self::from_bits_retain(flags as u32)
    }
}

impl From<Flags> for u32 {
    fn from(flags: Flags) -> Self {
        flags.bits()
    }
}

impl From<Flags> for u8 {
    fn from(flags: Flags) -> Self { flags.bits() as u8 }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone)]
/// Streaming format (single-planar)
pub struct Format {
    /// width in pixels
    pub width: u32,
    /// height in pixels
    pub height: u32,
    /// pixelformat code
    pub fourcc: FourCC,
    /// field order for interlacing
    pub field_order: FieldOrder,

    /// bytes per line
    pub stride: u32,
    /// maximum number of bytes required to store an image
    pub size: u32,

    /// flags set by the application or driver
    pub flags: Flags,

    /// supplements the pixelformat (fourcc) information
    pub colorspace: Colorspace,
    /// the way colors are mapped
    pub quantization: Quantization,
    /// the transfer function for the colorspace
    pub transfer: TransferFunction,
}

impl Format {
    /// Returns a capture format
    ///
    /// # Arguments
    ///
    /// * `width` - Width in pixels
    /// * `height` - Height in pixels
    /// * `fourcc` - Four character code (pixelformat)
    ///
    /// # Example
    ///
    /// ```
    /// use v4l::{Format, FourCC};
    /// let fmt = Format::new(640, 480, FourCC::new(b"YUYV"));
    /// ```
    pub const fn new(width: u32, height: u32, fourcc: FourCC) -> Self {
        Format {
            width,
            height,
            fourcc,
            field_order: FieldOrder::Any,
            stride: 0,
            size: 0,
            flags: Flags::empty(),
            colorspace: Colorspace::Default,
            quantization: Quantization::Default,
            transfer: TransferFunction::Default,
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "width          : {}", self.width)?;
        writeln!(f, "height         : {}", self.height)?;
        writeln!(f, "fourcc         : {}", self.fourcc)?;
        writeln!(f, "field          : {}", self.field_order)?;
        writeln!(f, "stride         : {}", self.stride)?;
        writeln!(f, "size           : {}", self.size)?;
        writeln!(f, "colorspace     : {}", self.colorspace)?;
        writeln!(f, "quantization   : {}", self.quantization)?;
        writeln!(f, "transfer       : {}", self.transfer)?;
        Ok(())
    }
}

impl From<v4l2_pix_format> for Format {
    fn from(fmt: v4l2_pix_format) -> Self {
        Self {
            width: fmt.width,
            height: fmt.height,
            fourcc: FourCC::from(fmt.pixelformat),
            field_order: FieldOrder::try_from(fmt.field).expect("Invalid field order"),
            stride: fmt.bytesperline,
            size: fmt.sizeimage,
            flags: Flags::from(fmt.flags),
            colorspace: Colorspace::try_from(fmt.colorspace).expect("Invalid colorspace"),
            quantization: Quantization::try_from(fmt.quantization).expect("Invalid quantization"),
            transfer: TransferFunction::try_from(fmt.xfer_func).expect("Invalid transfer function"),
        }
    }
}

impl From<Format> for v4l2_pix_format {
    fn from(format: Format) -> Self {
        Self {
            width: format.width,
            height: format.height,
            pixelformat: format.fourcc.into(),
            field: format.field_order as u32,
            bytesperline: format.stride,
            sizeimage: format.size,
            colorspace: format.colorspace as u32,
            flags: format.flags.into(),
            quantization: format.quantization as u32,
            xfer_func: format.transfer as u32,
            ..unsafe { mem::zeroed() }
        }
    }
}

/// streaming format (multi-planar)
#[derive(Debug, Copy, Clone)]
pub struct FormatMplane {
    /// width in pixels
    pub width: u32,
    /// height in pixels
    pub height: u32,
    /// pixelformat code
    pub fourcc: FourCC,
    /// field order for interlacing
    pub field_order: FieldOrder,

    pub plane_fmt: [FormatPlanePixItem; 8usize],

    pub num_planes: u8,

    /// flags set by the application or driver
    pub flags: Flags,

    /// supplements the pixelformat (fourcc) information
    pub colorspace: Colorspace,
    /// the way colors are mapped
    pub quantization: Quantization,
    /// the transfer function for the colorspace
    pub transfer: TransferFunction,
}

impl FormatMplane {
    pub const fn new(width: u32, height: u32, fourcc: FourCC) -> Self {
        FormatMplane {
            width,
            height,
            fourcc,
            field_order: FieldOrder::Any,
            plane_fmt: [FormatPlanePixItem { stride: 0, size: 0 }; 8usize],
            num_planes: 0,
            flags: Flags::empty(),
            colorspace: Colorspace::Default,
            quantization: Quantization::Default,
            transfer: TransferFunction::Default,
        }
    }
}

impl fmt::Display for FormatMplane {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "width          : {}", self.width)?;
        writeln!(f, "height         : {}", self.height)?;
        writeln!(f, "fourcc         : {}", self.fourcc)?;
        writeln!(f, "field_order    : {}", self.field_order)?;
        for i in 0..self.num_planes as usize {
            writeln!(f, "plane_fmt[{}]", i)?;
            write!(f, "{}", self.plane_fmt[i])?;
        }
        writeln!(f, "flags          : {}", self.flags)?;
        writeln!(f, "colorspace     : {}", self.colorspace)?;
        writeln!(f, "quantization   : {}", self.quantization)?;
        writeln!(f, "transfer       : {}", self.transfer)?;
        Ok(())
    }
}

impl From<v4l2_pix_format_mplane> for FormatMplane {
    fn from(fmt: v4l2_pix_format_mplane) -> Self {
        let mut plane_fmt = [FormatPlanePixItem {
            stride: fmt.plane_fmt[0].bytesperline,
            size: fmt.plane_fmt[0].sizeimage,
        }; 8usize];
        for i in 1..fmt.num_planes as usize {
            plane_fmt[i] = FormatPlanePixItem {
                stride: fmt.plane_fmt[i].bytesperline,
                size: fmt.plane_fmt[i].sizeimage,
            };
        }

        FormatMplane {
            width: fmt.width,
            height: fmt.height,
            fourcc: FourCC::from(fmt.pixelformat),
            field_order: FieldOrder::try_from(fmt.field).expect("Invalid field order"),
            flags: Flags::from(fmt.flags),
            colorspace: Colorspace::try_from(fmt.colorspace).expect("Invalid colorspace"),
            plane_fmt,
            num_planes: fmt.num_planes,
            quantization: Quantization::try_from(fmt.quantization).expect("Invalid quantization"),
            transfer: TransferFunction::try_from(fmt.xfer_func).expect("Invalid transfer function"),
        }
    }
}
impl From<FormatMplane> for v4l2_pix_format_mplane {
    fn from(format: FormatMplane) -> Self {
        let mut plane_fmt = [v4l2_plane_pix_format {
            bytesperline: format.plane_fmt[0].stride,
            sizeimage: format.plane_fmt[0].size,
            reserved: [0; 6usize],
        }; 8usize];
        for i in 1..format.num_planes as usize {
            plane_fmt[i] = v4l2_plane_pix_format {
                bytesperline: format.plane_fmt[i].stride,
                sizeimage: format.plane_fmt[i].size,
                reserved: [0; 6usize],
            };
        }

        v4l2_pix_format_mplane {
            width: format.width,
            height: format.height,
            pixelformat: format.fourcc.into(),
            field: format.field_order as u32,
            colorspace: format.colorspace as u32,
            plane_fmt,
            num_planes: format.plane_fmt.len() as u8,
            flags: format.flags.into(),
            quantization: format.quantization as u8,
            xfer_func: format.transfer as u8,
            ..unsafe { std::mem::zeroed() }
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct FormatPlanePixItem {
    /// bytes per line
    pub stride: u32,
    /// maximum number of bytes required to store an image
    pub size: u32,
}

impl fmt::Display for FormatPlanePixItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  stride         : {}", self.stride)?;
        writeln!(f, "  size           : {}", self.size)?;
        Ok(())
    }
}