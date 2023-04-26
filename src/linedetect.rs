use image::{ImageBuffer, Luma, GenericImageView};

#[derive(Debug)]
enum LineDetectError {
    NoLines,
}



fn detect_lines(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Result<usize, LineDetectError> {

    let image_width = image.width();
    let image_height = image.height();
    
    let pattern_width = image_width / 10;
    let pattern_height = 1;

    for y in 0..=(image_height-pattern_height) {
        for x in 0..=(image_width-pattern_width) {
            match image
                .view(x, y, pattern_width, pattern_height)
                .pixels()
                .find(|(_, _, l)| l.0[0] > 128u8) {
                    Some(_) => continue,
                    _ => println!("Black pixel @ x:{}, y:{}", x + pattern_width/2, y),
                }           
        }        
    }
    

    Ok(0)
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
    fn test_detect_one_line() {
        let image = generate_image_with_lines();
        detect_lines(image);
        //assert_eq!(10, detect_lines(image).unwrap());

    }

}