use super::image::{PixelsByRGBColor};

pub const RGB_ORIGIN : RGBColor = RGBColor(0, 0, 0);

#[derive(PartialEq,Clone,Copy,Debug,Eq,PartialOrd,Ord)]
pub struct RGBColor(pub u8,pub u8,pub u8);

impl RGBColor {
    pub fn squared_euclidean(&self, &RGBColor(rr, rg, rb): &Self) -> u32 {
        let &RGBColor(lr, lg, lb) = self;
        let dr = lr as i32 - rr as i32;
        let dg = lg as i32 - rg as i32;
        let db = lb as i32 - rb as i32;

        (dr * dr) as u32 + (dg * dg) as u32 + (db * db) as u32
    }
}

#[derive(Clone)]
pub struct RGBColorMean {
    pub sum: (u32,u32,u32),
    pub count: u32,
}

impl RGBColorMean {
    pub fn add(&mut self, pixel: &PixelsByRGBColor) {
        let (rsum, gsum, bsum) = self.sum;
        let RGBColor(red, green, blue) = pixel.color;
        self.count += pixel.pixels.len() as u32;
        self.sum = (
            rsum + (red as u32 * pixel.pixels.len() as u32), 
            gsum + (green as u32 * pixel.pixels.len() as u32), 
            bsum + (blue as u32 * pixel.pixels.len() as u32),
        );
    }

    pub fn to_mean(&self) -> Option<RGBColor> {
        if self.count == 0 {
            return None;
        }

        let (rsum, gsum, bsum) = self.sum;
        Some(RGBColor((rsum / self.count) as u8, (gsum / self.count) as u8, (bsum / self.count) as u8))
    }

    pub fn new() -> Self {
        RGBColorMean {
            sum: (0, 0, 0),
            count: 0,
        }
    }
}

