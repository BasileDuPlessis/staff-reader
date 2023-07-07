mod staffmatcher;
mod linedetect;
mod staff;

fn main() {
    println!("Hello world!");
}

#[cfg(test)]
mod tests {

    use image::{ImageBuffer, Luma};
    use crate::staffmatcher::StaffMatcher;

    use super::*;


    fn load_image(image_name: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {

        let image = image::io::Reader::open(image_name).unwrap()
        .decode().unwrap();

        image.into_luma8()
    }

    fn match_on_line_pattern(
        pattern: linedetect::Pattern,
        image_luma8: &ImageBuffer<Luma<u8>, Vec<u8>>
    ) -> StaffMatcher {

        let img_pattern_match = linedetect::ImgPatternMatcher::new(image_luma8, pattern);
        let matched_pixels = img_pattern_match.iter();
    
        let mut matcher = staffmatcher::StaffMatcher::new(
            image_luma8.width() as usize,
            image_luma8.height() as usize
        );
    
        for (x, y) in matched_pixels {
            matcher.add_black_pixel(x as usize, y as usize);
        }

        matcher

    }
    
    #[test]
    fn test_1_staff_image()  {

        let image_luma8 = load_image("staff_images/1-empty-staff.jpg");

        let pattern = linedetect::Pattern::Staff(5, 1, vec![0; 5]);
        
        let mut matcher = match_on_line_pattern(pattern, &image_luma8);

        let staff:Vec<staff::Staff> = matcher.iter().collect();

        assert_eq!(1, staff.len());   

    }

    #[test]
    fn test_staff_image_with_clef()  {

        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

        let image_luma8 = load_image("staff_images/staff-with-clef.jpeg");
        
        let pattern = linedetect::Pattern::Staff(5, 3, vec![0; 15]);
        
        let mut matcher = match_on_line_pattern(pattern, &image_luma8);

        let staff:Vec<staff::Staff> = matcher.iter().collect();

        assert_eq!(1, staff.len());   

    }

}