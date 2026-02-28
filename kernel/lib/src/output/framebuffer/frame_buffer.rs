use core::{fmt, ptr};

use noto_sans_mono_bitmap::{
    FontWeight, RasterHeight, RasterizedChar, get_raster, get_raster_width,
};

use bootloader_api::info::{FrameBufferInfo, PixelFormat};

#[derive(PartialEq, Eq, Debug)]
pub enum FrameBufferError {
    NotPresent,
    InfoNotPresent,
}

const LINE_SPACING: usize = 2;
const LETTER_SPACING: usize = 0;
const BORDER_PADDING: usize = 1;

mod font_constants {
    use super::*;

    pub const CHAR_RASTER_HEIGHT: RasterHeight = RasterHeight::Size16;

    pub const CHAR_RASTER_WIDTH: usize = get_raster_width(FontWeight::Regular, CHAR_RASTER_HEIGHT);

    pub const BACKUP_CHAR: char = '?';

    pub const FONT_WEIGHT: FontWeight = FontWeight::Regular;
}

pub enum FrameBufferColour {
    Red,
    Green,
    Blue,
    White,
    Yellow,
    Purple,
    Brown,
}

impl FrameBufferColour {
    pub fn to_bgr(intensity: u8, colour: &Self) -> [u8; 4] {
        match colour {
            FrameBufferColour::Red => [0, 0, intensity, 0],
            FrameBufferColour::Green => [0, intensity, 0, 0],
            FrameBufferColour::Blue => [intensity, 0, 0, 0],
            FrameBufferColour::White => [intensity, intensity, intensity, 0],
            FrameBufferColour::Yellow => [intensity, intensity, 0, 0],
            FrameBufferColour::Purple => [intensity, 0, intensity, 0],
            FrameBufferColour::Brown => [0, intensity, intensity, 0],
        }
    }
}

fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(
            c,
            font_constants::FONT_WEIGHT,
            font_constants::CHAR_RASTER_HEIGHT,
        )
    }
    get(c)
        .unwrap_or_else(|| get(font_constants::BACKUP_CHAR).expect("Should get backup char raster"))
}

pub struct FrameBufferWriter {
    framebuffer: Option<&'static mut [u8]>,
    info: Option<FrameBufferInfo>,
    x: usize,
    y: usize,
    current_colour: FrameBufferColour,
}

impl FrameBufferWriter {
    pub fn set(&mut self, framebuffer: &'static mut [u8], info: FrameBufferInfo) {
        self.framebuffer = Some(framebuffer);
        self.info = Some(info);
    }
    pub fn new() -> Self {
        Self {
            framebuffer: None,
            info: None,
            x: 0,
            y: 0,
            current_colour: FrameBufferColour::White,
        }
    }
    pub fn change_colour(&mut self, colour: FrameBufferColour) {
        self.current_colour = colour;
    }
    fn newline(&mut self) {
        self.y += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return()
    }

    fn carriage_return(&mut self) {
        self.x = BORDER_PADDING;
    }

    pub fn clear(&mut self) -> Result<(), FrameBufferError> {
        self.x = BORDER_PADDING;
        self.y = BORDER_PADDING;
        if let Some(framebuffer) = self.framebuffer.as_mut() {
            framebuffer.fill(0);
            Ok(())
        } else {
            Err(FrameBufferError::NotPresent)
        }
    }

    fn width(&self) -> Result<usize, FrameBufferError> {
        if let Some(info) = self.info {
            Ok(info.width)
        } else {
            Err(FrameBufferError::InfoNotPresent)
        }
    }
    fn height(&self) -> Result<usize, FrameBufferError> {
        if let Some(info) = self.info {
            Ok(info.height)
        } else {
            Err(FrameBufferError::InfoNotPresent)
        }
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                let new_xpos = self.x + font_constants::CHAR_RASTER_WIDTH;
                if new_xpos >= self.width().expect("Can't get framebuffer width") {
                    self.newline();
                }
                let new_ypos = self.y + font_constants::CHAR_RASTER_HEIGHT.val() + BORDER_PADDING;
                if new_ypos >= self.height().expect("Can't get framebuffer height") {
                    self.clear().expect("Framebuffer was unable to clear");
                }
                self.write_rendered_char(get_char_raster(c));
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: RasterizedChar) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x + x, self.y + y, *byte);
            }
        }
        self.x += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let (stride, mut pixel_format, bytes_per_pixel): (usize, PixelFormat, usize) =
            if let Some(info) = self.info {
                (info.stride, info.pixel_format, info.bytes_per_pixel)
            } else {
                panic!("Info is not present");
            };

        let pixel_offset = y * stride + x;
        let color = match pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity, 0],
            /*
            PixelFormat::Bgr => [
                self.current_colour.b,
                self.current_colour.g,
                self.current_colour.r,
                0,
            ],
            */
            PixelFormat::Bgr => FrameBufferColour::to_bgr(intensity, &self.current_colour),
            //PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            //PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            other => {
                pixel_format = PixelFormat::Rgb;
                panic!("pixel format {:?} not supported in logger", other)
            }
        };
        let bytes_per_pixel = bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        if let Some(framebuffer) = self.framebuffer.as_mut() {
            framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
                .copy_from_slice(&color[..bytes_per_pixel]);
            let _ = unsafe { ptr::read_volatile(&framebuffer[byte_offset]) };
        }
    }
}

impl fmt::Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
