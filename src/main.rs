/*
Rust: Image processing with rayon
Copyright (C) 2023 Aryo Pehlewan aryopehlewan@hotmail.com
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use rayon::prelude::*;

type Pixel = (u8, u8, u8);  // Red, Green, Blue
type Image = Vec<Vec<Pixel>>;

// simple grayscale filter
fn apply_filter(pixel: &Pixel) -> Pixel {
    let (r, g, b) = *pixel;

    // Compute the average of the red, green and blue values (use integer arithmetic)
    let grey = (r as u16 + g as u16 + b as u16) / 3;

    // Return a new pixel where all color channels have the grey value
    // The as u8 cast is safe because the average of three u8 values will always be within u8 range.
    (grey as u8, grey as u8, grey as u8)
}

fn main() {
    let image: Image = vec![vec![(10, 20, 30); 800]; 600];

    let processed_image: Image = image.par_iter()
        .map(|row| {
            row.iter()
               .map(|pixel| apply_filter(pixel))
               .collect()
        })
        .collect();

    println!("Processed image size: {}x{}", processed_image.len(), processed_image[0].len());
}