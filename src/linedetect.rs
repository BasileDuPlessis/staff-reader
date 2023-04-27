use image::{ImageBuffer, Luma, GenericImageView};

#[derive(Debug)]
enum LineDetectError {
    NoLines,
}

struct ImgPatternMatcher {
    pattern_width: u32,
    pattern_height: u32,
    image: ImageBuffer<Luma<u8>, Vec<u8>>,
}

type Pixel = (u32, u32);

impl ImgPatternMatcher {
    fn new(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImgPatternMatcher {
        ImgPatternMatcher {
            pattern_width: 5,
            pattern_height: 1,
            image
        }
    }
    fn iter(&self) -> MatchedPixels {
        MatchedPixels {
            img_pattern_match: self,
            x: 0,
            y: 0,
        }
    }
}

struct MatchedPixels<'a> {
    img_pattern_match: &'a ImgPatternMatcher,
    x: u32,
    y: u32,
}

impl<'a> Iterator for MatchedPixels<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {

        loop {
            let result= match self.img_pattern_match.image
            .view(self.x, self.y, self.img_pattern_match.pattern_width, self.img_pattern_match.pattern_height)
            .pixels()
            .find(|(_, _, l)| l.0[0] > 128u8) {
                Some(_) => None,
                _ => Some((self.x, self.y)),
            };

            if self.x == (self.img_pattern_match.image.width() - self.img_pattern_match.pattern_width) {
                self.x = 0;
                if self.y <= (self.img_pattern_match.image.height() - self.img_pattern_match.pattern_height) {
                    self.y += 1;
                } else {
                    return None;
                }
            } else {
                self.x += 1;
            }

            match result {
                Some(_) => return result,
                _ => continue,
            }
        }

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn generate_image_with_lines() -> ImageBuffer<Luma<u8>, Vec<u8>>  {
        let mut image = 
            ImageBuffer::from_fn(50, 10, |x, y|
                if y==5 {
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
        let img_pattern_match = ImgPatternMatcher::new(image);
        let mut iter = img_pattern_match.iter();

        while let Some(pixel) = iter.next() {
            println!("Matched pixel: {}, {}", pixel.0, pixel.1);
        }
    }


}