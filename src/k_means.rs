use crate::commons::image::{Image, Pixel, PixelsByRGBColor};
use crate::commons::rgb::{RGBColor, RGBColorMean};
use rand::Rng;
use std::cmp::max;

const LAST_ITES: usize = 8;

pub struct KMeans {
    width: u32,
    height: u32,
    pixels: Vec<PixelsByRGBColor>,
    means: Vec<RGBColor>,
    deduplicate: bool,
}

pub struct KMeansReturn {
    pub image: Image,
    pub means: Vec<RGBColor>,
}

impl KMeans {
    pub fn new(k: u8, deduplicate: bool, image: Image) -> Self {
        let Image {width, height, ..} = image;
        Self {
            width,
            height,
            pixels: image.pixels_by_color(),
            means: Self::random_k_means(k),
            deduplicate,
        }
    }

    pub fn run(mut self) -> KMeansReturn {
        let mut diffs = [0u32; LAST_ITES];
        let mut dedupped = false;
        for i in 0..512 {
            let (mut new_means, diff) = self.iterate_means();
            if self.deduplicate && i % 8 == 7 {
                let (dedupped_new_means, d) = self.dedup();
                new_means = dedupped_new_means;
                dedupped = d;
            }
            self.means = new_means;
            diffs[i % LAST_ITES] = diff;
            if i >= 16 &&
                !(self.deduplicate && dedupped) && 
                !diffs.iter().any(|&diff| diff >= 100u32) {
                break;
            }
        }
        self.build_return()
    }

    fn dedup(&self) -> (Vec<RGBColor>, bool) {
        let mut result = vec![];
        let mut dedupped = false;
        for mean in self.means.iter() {
            let mut saw_similar = false;
            for saw_mean in result.iter() {
                if mean.squared_euclidean(&saw_mean) < 100 {
                    saw_similar = true;
                    break;    
                }
            }
            if !saw_similar {
                result.push(*mean);
            } else {
                dedupped = true;
                result.push(Self::random_mean());
            }
        }
        (result, dedupped)
    }

    fn iterate_means(&self) -> (Vec<RGBColor>, u32) {
        let mut means_means = vec![RGBColorMean::new(); self.means.len()];
        for pixel in self.pixels.iter() {
            let idx = self.find_nearest(&pixel);
            means_means[idx].add(&pixel);
        }
        let mut diff = 0u32;
        (
            means_means.iter().enumerate()
                .map(|(idx, mean_mean)| {
                    let new_mean_option = mean_mean.to_mean();
                    if let Some(mean) = new_mean_option {
                        diff = max(diff, self.means[idx].squared_euclidean(&mean));
                        mean
                    } else {
                        Self::random_mean()
                    }
                })
                .collect(),
            diff,
        )
    }
    
    fn find_nearest(&self, pixel: &PixelsByRGBColor) -> usize {
        self.means.iter().enumerate()
            .min_by_key(|(_, mean)| mean.squared_euclidean(&pixel.color))
            .unwrap().0
    }

    fn build_return(self) -> KMeansReturn {
        let image = self.to_image();
        let means = self.means;
        KMeansReturn {image, means}
    }

    fn to_image(&self) -> Image {
        let pixels = self.pixels.iter()
            .flat_map(|pixel| {
                let idx = self.find_nearest(&pixel);
                let RGBColor(red, green, blue) = self.means[idx];
                pixel.pixels.iter()
                    .map(move |pixel| {
                        let &Pixel {x, y, ..} = pixel;
                        Pixel {x, y, color: RGBColor(red, green, blue)}
                    })
            })
            .collect();
        Image {width: self.width, height: self.height, pixels}
    }

    fn random_mean() -> RGBColor {
        let mut rng = rand::thread_rng();
        RGBColor(rng.gen(), rng.gen(), rng.gen())
    }

    fn random_k_means(k: u8) -> Vec<RGBColor> {
        (0..k).map(|_| Self::random_mean()).collect()
    }
}

