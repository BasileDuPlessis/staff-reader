#[derive(Clone, Debug, PartialEq)]
enum StaffZone {
    Line(usize),
    Spacing(usize),    
}

#[derive(Debug)]
struct StaffMatcher {
    width: usize,
    height: usize,
    pixel_arr: Vec<bool>,
    zone_arr: Vec<StaffZone>,
}

impl StaffMatcher {
    fn new(width: usize, height: usize) -> StaffMatcher {
        StaffMatcher {
            width,
            height,
            pixel_arr: vec![false; width * height],
            zone_arr: Vec::new()
        }        
    }

    fn prepare(&mut self) -> &StaffMatcher {
        if self.width == 0 || self.height == 0 {
            panic!("Cannot match Staff on an empty area");
        }

        self.zone_arr = Vec::new();

        let count_true: Vec<usize> = self.pixel_arr
            .chunks(self.width)
            .map(|v| v.iter().filter(|&p| *p).count())
            .collect();

        for density in count_true.iter() {
            let last_zone = self.zone_arr.last_mut();
            let is_line = *density * 2 >= self.width;
  
            match last_zone {
                Some(StaffZone::Line(ref mut line_size)) if is_line => 
                    *line_size += 1,
                Some(StaffZone::Line(..)) if !is_line => 
                    self.zone_arr.push(StaffZone::Spacing(1)),
                Some(StaffZone::Spacing(ref mut spacing_size)) if !is_line => 
                    *spacing_size += 1,
                _ =>
                    if is_line {
                        self.zone_arr.push(StaffZone::Line(1));
                    } else {
                        self.zone_arr.push(StaffZone::Spacing(1))
                    },
            }
        };

        self
    }


    fn match_staff(&self) -> Result<Vec<Vec<StaffZone>>, StaffMatchError> {

        let mut staff_arr:Vec<Vec<StaffZone>> = Vec::new();

        for zone in &self.zone_arr {
            match zone {
                StaffZone::Line(size) => {
                    for staff in &mut staff_arr {
                        match staff.as_slice() {
                            [.., StaffZone::Line(s), StaffZone::Spacing(_)] => {
                                if s == size {
                                    staff.push(zone.clone());
                                } else {
                                    log::trace!("Unable to push Line of size: {} in StaffZone made with Line of size: {}", size, s);
                                }
                            }   
                            _ => (),
                        }
                    }
                   staff_arr.push(vec!(zone.clone())); 
                },
                StaffZone::Spacing(size) => {
                    for staff in &mut staff_arr {
                        match staff.as_slice() {
                            [StaffZone::Line(_)] => staff.push(zone.clone()),
                            [.., StaffZone::Spacing(s), StaffZone::Line(_)] if s == size =>
                                staff.push(zone.clone()),
                            _ => ()
                        }
                    }
                }
            }
        };

        let result:Vec<Vec<StaffZone>> = staff_arr.into_iter().filter(|staff| staff.len() == 9).collect();
        
        if result.len() > 0 {
            Ok(result)
        } else {
            Err(StaffMatchError::NoMatch)
        }
        
    }

    fn add_black_pixel(&mut self, x: &usize, y: &usize) {
        if let Some(pixel) = self.pixel_arr.get_mut(y * self.width + x) {
            *pixel = true;
        }
    }
}

#[derive(PartialEq, Debug)]
enum StaffMatchError {
    NoMatch,
}


#[cfg(test)]
mod tests {

    fn log_init() {
        let _ = env_logger::builder()
            .target(env_logger::Target::Stdout)
            .filter_level(log::LevelFilter::Trace)
            .is_test(true)
            .try_init();
    }

    use super::{StaffMatchError, StaffMatcher, StaffZone};
    
    fn generate_staff_image(pattern: Vec<i32>, width: usize) 
        -> (Vec<(usize, usize)>, usize) {
        
        let mut result:Vec<(usize, usize)> = Vec::new();
        let mut y:usize = 0;

        for p in pattern {
            let height = p.abs() as usize;
            for y in y..(y + height) {
                 if p > 0 {
                     (0..width).for_each(|x| result.push((x, y)));
                 }
            }
            y += height;
        }
        
        (result, y)
    }

    #[test]
    fn test_prepare_staff() {
        let width = 10;
        let pattern = vec!(-1, -2, 3, 4, -5, 6, -7);

        let (pixel_arr, height) = generate_staff_image(
            pattern,
            width
        );

        let mut matcher = StaffMatcher::new(
            width,
            height
        );

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(x, y));
        
        let result = vec!(
            StaffZone::Spacing(3),
            StaffZone::Line(7),
            StaffZone::Spacing(5),
            StaffZone::Line(6),
            StaffZone::Spacing(7),
        );

        matcher.prepare();

        assert_eq!(result, matcher.zone_arr);

    }


    #[test]
    #[should_panic]
    fn test_prepare_panic_if_null_width() {
        let mut matcher = StaffMatcher::new(0, 10);
        matcher.prepare();
    }

    #[test]
    #[should_panic]
    fn test_prepare_panic_if_null_height() {
        let mut matcher = StaffMatcher::new(10, 0);
        matcher.prepare();
    }

    #[test]
    fn test_add_point() {

        let mut matcher = StaffMatcher::new(10, 1);

        for x in 0..10 {
            matcher.add_black_pixel(&x, &0);
        }

        assert_eq!(vec![true; 10], matcher.pixel_arr);      
    }

    #[test]
    fn test_staff_matched() {

        let mut matcher = StaffMatcher::new(10, 40);
        
        matcher.zone_arr = vec!(
            StaffZone::Spacing(2),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(2),
        );
        
        let result = vec!(vec!(
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
        ));

        assert_eq!(Ok(result), matcher.match_staff());
    }

    #[test]
    fn test_two_staff_matched() {
                
        log_init();
        
        let width = 10;
        let pattern = vec!(-2, 3, -4, 3, -5, 3, -5, 3, -5, 3, -5, 3, -2, -4, 2, -2, 2, -2, 2, -2, 2, -2, 2);

        let (pixel_arr, height) = generate_staff_image(
            pattern,
            width
        );

        let mut matcher = StaffMatcher::new(
            width,
            height
        );

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(x, y));
        
        let result = vec!(vec!(
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
            StaffZone::Spacing(5),
            StaffZone::Line(3),
        ), vec!(
            StaffZone::Line(2),
            StaffZone::Spacing(2),
            StaffZone::Line(2),
            StaffZone::Spacing(2),
            StaffZone::Line(2),
            StaffZone::Spacing(2),
            StaffZone::Line(2),
            StaffZone::Spacing(2),
            StaffZone::Line(2),
        ));

        assert_eq!(Ok(result), matcher.prepare().match_staff());
    }

    #[test]
    fn test_staff_not_matched() {
        let width = 10;
        let pattern = vec!(-2, 3, -4, 3, -5, 3, -5, 3, -5, 3, -2);

        let (pixel_arr, height) = generate_staff_image(
            pattern,
            width
        );

        let mut matcher = StaffMatcher::new(
            width,
            height
        );

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(x, y));

        assert_eq!(Err(StaffMatchError::NoMatch), matcher.prepare().match_staff());
    }

}