use image::{ImageBuffer, Luma, GenericImageView};


pub enum Pattern {
    Staff(u32, u32, Vec<u8>),
}

enum MatchingMode {
    Perfect,
}

pub struct ImgPatternMatcher<'a> {
    pattern_width: u32,
    pattern_height: u32,
    pattern_vec: Vec<u8>,
    image: &'a ImageBuffer<Luma<u8>, Vec<u8>>,
    matching_mode: MatchingMode
}

type Pixel = (u32, u32);

impl<'a> ImgPatternMatcher<'a> {
    pub fn new(image: &ImageBuffer<Luma<u8>, Vec<u8>>, pattern: Pattern) -> ImgPatternMatcher {
        match pattern {
            Pattern::Staff(w, h, vec) => {
                assert!(w * h == vec.len() as u32, "Pattern size do not match pattern content");
                assert!(w % 2 == 1 && h % 2 == 1, "Pattern width and height should be odd");
                assert!(image.width() > w && image.height() > h, "Pattern cannot be larger than image");
                ImgPatternMatcher {
                    pattern_width: w,
                    pattern_height: h,
                    pattern_vec: vec,
                    image,
                    matching_mode: MatchingMode::Perfect,
                }
            },
            _ => todo!("Pattern not implemented")
        }        
    }
    pub fn iter(&self) -> MatchedPixels {
        MatchedPixels {
            matcher: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct MatchedPixels<'a> {
    matcher: &'a ImgPatternMatcher<'a>,
    x: u32,
    y: u32,
}

impl<'a> Iterator for MatchedPixels<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {

        let xmax = self.matcher.image.width() - self.matcher.pattern_width;
        let ymax = self.matcher.image.height() - self.matcher.pattern_height;

        loop {

            let matched_pixel= match self.matcher.image
                .view(self.x, self.y, self.matcher.pattern_width, self.matcher.pattern_height)
                .pixels()
                .zip(self.matcher.pattern_vec.iter())
                .find(
                    |((_, _, subimage_pixel), pattern_pixel)|
                    match self.matcher.matching_mode {
                        MatchingMode::Perfect => {
                            subimage_pixel.0[0] > 128u8 && **pattern_pixel < 128u8
                            ||
                            subimage_pixel.0[0] < 128u8 && **pattern_pixel > 128u8 
                        },
                    }                    
                ) {
                    Some(_) => None,
                    None => Some((self.x + (self.matcher.pattern_width - 1) / 2, self.y + (self.matcher.pattern_height - 1) / 2)),
                };

            if self.x == xmax {
                self.x = 0;
                if self.y == ymax {
                    return None;
                }
                self.y += 1;
            } else {
                self.x += 1;
            }

            match matched_pixel {
                None => continue,
                _ => return matched_pixel,
            }
        }

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn generate_image_with_lines() -> ImageBuffer<Luma<u8>, Vec<u8>>  {
        let image = 
            ImageBuffer::from_fn(11, 5, |_, y|
                if y==2 {
                    image::Luma([0u8])
                } else {
                    image::Luma([255u8])
                }
            );

        image
    }

    #[test]
    fn test_iter_on_matcher() {
        let image = generate_image_with_lines();
        let pattern = Pattern::Staff(5, 1, vec![0; 5]);
        let img_pattern_match = ImgPatternMatcher::new(&image, pattern);
        let matched_pixels:Vec<(u32, u32)> = img_pattern_match.iter().collect();

        assert_eq!(vec![(2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (8, 2)], matched_pixels);
    }

    #[test]
    #[should_panic]
    fn test_matcher_panic_if_pattern_size_incorrect() {
        let image = generate_image_with_lines();
        let pattern = Pattern::Staff(4, 1, vec![0; 5]);
        ImgPatternMatcher::new(&image, pattern);
    }

    #[test]
    #[should_panic]
    fn test_matcher_panic_if_pattern_size_greater_than_image_size() {
        let image = generate_image_with_lines();
        let pattern = Pattern::Staff(15, 1, vec![0; 15]);
        ImgPatternMatcher::new(&image, pattern);
    }

    #[test]
    #[should_panic]
    fn test_matcher_panic_if_pattern_size_is_even() {
        let image = generate_image_with_lines();
        let pattern = Pattern::Staff(4, 2, vec![0; 8]);
        ImgPatternMatcher::new(&image, pattern);
    }




}