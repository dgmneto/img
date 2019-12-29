use super::rgb::RGBColor;
use image::{open, GenericImageView, ImageBuffer, Pixel as ImagePixel};
use std::path::PathBuf;
use itertools::Itertools;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

#[derive(Clone,Debug,Copy)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: RGBColor,
}

pub struct PixelsByRGBColor {
    pub color: RGBColor,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn from_file(path: &PathBuf) -> Result<Image, failure::Error> {
        let image = open(path)?;
        let width = image.width();
        let height = image.height();
        let pixels = image.pixels().map(|(x, y, pixel)| {
                let (red, green, blue, _) = pixel.channels4();
                Pixel {x, y, color: RGBColor(red, green, blue)}
            })
            .collect();
        Ok(Image { width, height, pixels })
    }
    
    pub fn pixels_by_color(self) -> Vec<PixelsByRGBColor> {
        self.pixels.into_iter()
            .sorted_by_key(|&Pixel {color, ..}| color)
            .group_by(|&Pixel {color, ..}| color)
            .into_iter()
            .map(|(color, pixels)| PixelsByRGBColor {color, pixels: pixels.collect()})
            .collect()
    }

    pub fn save_to_path(self, path: &PathBuf) -> Result<(), std::io::Error> {
        let Image {width, height, pixels} = self;
        let mut image_buff = ImageBuffer::new(width, height);
        for Pixel {x, y, color: RGBColor(red, green, blue)} in pixels {
            image_buff.put_pixel(x, y, image::Rgb([red, green, blue]));
        }
        image_buff.save(path)
    }
}