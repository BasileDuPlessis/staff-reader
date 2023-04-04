struct Staff {
}

#[derive(Clone, Debug, PartialEq)]
enum StaffZone {
    Line(usize),
    Spacing(usize),    
}

impl Staff {

}

#[derive(Debug)]
struct StaffMatcher {
    width: usize,
    height: usize,
    pixel_arr: Vec<bool>,
}

impl StaffMatcher {
    fn new(width: usize, height: usize) -> StaffMatcher {
        StaffMatcher {
            width,
            height,
            pixel_arr: vec![false; width * height],
        }        
    }
    fn match_staff(&self) -> Result<Vec<Vec<StaffZone>>, StaffMatchError> {

        if self.width == 0 || self.height == 0 {
            return Err(StaffMatchError::IsEmpty);
        }

        let count_arr: Vec<usize> = self.pixel_arr
            .chunks(self.width)
            .map(|v| v.iter().filter(|&p| *p).count())
            .collect();


        let mut match_arr:Vec<StaffZone> = Vec::new();

        for d in count_arr.iter() {
            let last_zone = match_arr.last_mut();
            let is_line = *d * 2 >= self.width;
  
            match last_zone {
                Some(StaffZone::Line(ref mut line_size)) if is_line => 
                    *line_size += 1,
                Some(StaffZone::Line(..)) if !is_line => 
                    match_arr.push(StaffZone::Spacing(1)),
                Some(StaffZone::Spacing(ref mut spacing_size)) if !is_line => 
                    *spacing_size += 1,
                _ =>
                    if is_line {
                        match_arr.push(StaffZone::Line(1));
                    } else {
                        match_arr.push(StaffZone::Spacing(1))
                    },
            }
        };

        let mut staff_arr:Vec<Vec<StaffZone>> = Vec::new();

        for zone in match_arr {
            match zone {
                StaffZone::Line(size) => {
                    for staff in &mut staff_arr {
                        match staff.as_slice() {
                            [.., StaffZone::Line(s), StaffZone::Spacing(_)] if s == &size =>
                                staff.push(zone.clone()),
                            _ => (),
                        }
                    }
                   staff_arr.push(vec!(zone.clone())); 
                },
                StaffZone::Spacing(size) => {
                    for staff in &mut staff_arr {
                        match staff.as_slice() {
                            [StaffZone::Line(_)] => staff.push(zone.clone()),
                            [.., StaffZone::Spacing(s), StaffZone::Line(_)] if s == &size =>
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
    IsEmpty
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

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
    fn test_empty_row_arr() {

        let matcher = StaffMatcher::new(0, 0);

        assert_eq!(Err(StaffMatchError::IsEmpty), matcher.match_staff());

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
        
        let width = 10;
        let pattern = vec!(-2, 3, -5, 3, -5, 3, -5, 3, -5, 3, -2);

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
        ));


        assert_eq!(Ok(result), matcher.match_staff());
    }
}