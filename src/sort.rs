use crate::commons::image::{Pixel, Image};
use crate::commons::rgb::RGB_ORIGIN;
use itertools::Itertools;
use rand::prelude::*;

pub struct SortReturn {
    pub image: Image,
}

pub struct Sort {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    rows_to_sort: f64,
}

impl Sort {
    pub fn new(image: Image, rows_to_sort: f64) -> Self {
        let Image {width, height, pixels} = image;
        Self {width, height, pixels, rows_to_sort}
    }

    pub fn run(mut self) -> SortReturn {
        self.pixels = self.pixels.iter()
            .sorted_by_key(|&Pixel {x, ..}| x)
            .group_by(|&Pixel {x, ..}| x)
            .into_iter()
            .flat_map(|(&x, pixels)| self.sort_row(x, pixels))
            .collect();
        let image = Image {width: self.width, height: self.height, pixels: self.pixels};
        SortReturn {image}
    }

    fn sort_row<'a>(&self, 
            x: u32, 
            pixels: impl IntoIterator<Item = &'a Pixel>,
        ) -> Vec<Pixel> {
        let toss: f64 = thread_rng().gen();
        if toss <= self.rows_to_sort {
            pixels.into_iter()
                .sorted_by_key(|Pixel {color, ..}| color.squared_euclidean(&RGB_ORIGIN))
                .enumerate()
                .map(move |(y, &Pixel {color, ..})| Pixel {x, y: y as u32, color})
                .collect()
        } else {
            pixels.into_iter().map(|&pixel| pixel).collect()
        }
    }
}

