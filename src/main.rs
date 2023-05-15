mod staffmatcher;
mod linedetect;

fn main() {
    println!("Hello world!");
}

#[test]
fn test_empty_staves_image()  {

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();


    let image = image::io::Reader::open("staff_images/1-empty-stave.jpg").unwrap()
        .decode().unwrap();

    let image_luma8 = image.into_luma8();


    let pattern = linedetect::Pattern::Staff(5, 1, vec![0; 5]);
    let img_pattern_match = linedetect::ImgPatternMatcher::new(&image_luma8, pattern);
    let matched_pixels = img_pattern_match.iter();

    let mut matcher = staffmatcher::StaffMatcher::new(
        image_luma8.width() as usize,
        image_luma8.height() as usize
    );

    for (x, y) in matched_pixels {
        matcher.add_black_pixel(x as usize, y as usize);
    }

    let staff:Vec<staffmatcher::Staff> = matcher.iter().collect();

    assert_eq!(1, staff.len()); 
    

}
