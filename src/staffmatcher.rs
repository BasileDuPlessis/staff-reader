use log::{trace};

#[derive(Clone, Debug, PartialEq)]
enum Area {
    Line(usize),
    Spacing(usize),    
}
#[derive(Debug, Clone)]
pub struct Staff {
    areas: Vec<Area>,
    complete: bool,
}

enum StaffMatchError {
    NoMatch,
    StaffComplete,
}

impl Staff {
    fn new(area: &Area) -> Staff {
        Staff {
            areas: vec![area.clone()],
            complete: false,
        }
    }

    fn area_size_fit(&self, s_prev: usize, s_to_match: usize) -> bool {
        s_to_match >= s_prev - 1 && s_to_match <= s_prev + 1
    }

    fn add_area(&mut self, area_to_match: &Area) -> Result<(), StaffMatchError> {

        if self.complete {
            return Err(StaffMatchError::StaffComplete);
        }
 
        let area_slice = self.areas.as_slice();

        match area_to_match {
            &Area::Line(s1)=> {
                match area_slice {
                    [.., Area::Line(s2), Area::Spacing(_)] if self.area_size_fit(*s2, s1) =>
                        self.areas.push(area_to_match.clone()),
                    _ => return Err(StaffMatchError::NoMatch),
                }
            }
            &Area::Spacing(s1) => {
                match area_slice {
                    [Area::Line(_)] => self.areas.push(area_to_match.clone()),
                    [.., Area::Spacing(s2), Area::Line(_)] if self.area_size_fit(*s2, s1) =>
                        self.areas.push(area_to_match.clone()),
                    _ => return Err(StaffMatchError::NoMatch),
                }
            }
        }

        if self.areas.len() == 9 {
            self.complete = true;
        }

        Ok(())

    }
}

#[derive(Debug)]
pub struct StaffMatcher {
    width: usize,
    height: usize,
    pixel_arr: Vec<bool>,
    area_vec: Vec<Area>,
}

pub struct MatchedStaffs<'a> {
    matcher: &'a StaffMatcher,
    index: usize,
}

impl StaffMatcher {
    pub fn new(width: usize, height: usize) -> StaffMatcher {
        StaffMatcher {
            width,
            height,
            pixel_arr: vec![false; width * height],
            area_vec: Vec::new()
        }        
    }

    fn prepare(&mut self) -> &StaffMatcher {
        if self.width == 0 || self.height == 0 {
            panic!("Cannot match Staff on an empty area");
        }

        self.area_vec = Vec::new();

        let count_true: Vec<usize> = self.pixel_arr
            .chunks(self.width)
            .map(|v| v.iter().filter(|&p| *p).count())
            .collect();

        for density in count_true.iter() {
            let last_zone = self.area_vec.last_mut();
            let is_line = *density * 2 >= self.width;
  
            match last_zone {
                Some(Area::Line(ref mut line_size)) if is_line => 
                    *line_size += 1,
                Some(Area::Line(..)) if !is_line => 
                    self.area_vec.push(Area::Spacing(1)),
                Some(Area::Spacing(ref mut spacing_size)) if !is_line => 
                    *spacing_size += 1,
                _ =>
                    if is_line {
                        self.area_vec.push(Area::Line(1));
                    } else {
                        self.area_vec.push(Area::Spacing(1))
                    },
            }
        };

        self
    }

    pub fn add_black_pixel(&mut self, x:usize, y:usize) {
        if let Some(pixel) = self.pixel_arr.get_mut(y * self.width + x) {
            *pixel = true;
        }
    }

    pub fn iter(&mut self) -> MatchedStaffs {
        self.prepare();
        MatchedStaffs {
            matcher: self,
            index: 0,
        }
    }
}


impl<'a> Iterator for MatchedStaffs<'a> {
    type Item = Staff;

    fn next(&mut self) -> Option<Self::Item> {

        let mut area_iter = self.matcher.area_vec.iter().skip(self.index);

        let mut staff_vec:Vec<Staff> = Vec::new();

        while let Some(area) = area_iter.next()  {
            self.index += 1;

            for staff in staff_vec.iter_mut() {
                
                match staff.add_area(area) {
                    Err(StaffMatchError::NoMatch) => log::debug!("Area {:?} do not match staff {:?}", area, staff),
                    Err(StaffMatchError::StaffComplete) => log::debug!("Area {:?} do not match staff {:?} because it is already complete", area, staff),
                    _ => log::debug!("Add area {:?} to staff {:?}", area, staff),
                }

                if staff.complete {
                    return Some(staff.clone());
                }
            }

            staff_vec.push(Staff::new(area));
        }

        None
        
    }
}



#[cfg(test)]
mod tests {

    use super::{StaffMatcher, Area};
    
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

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(*x, *y));
        
        let result = vec!(
            Area::Spacing(3),
            Area::Line(7),
            Area::Spacing(5),
            Area::Line(6),
            Area::Spacing(7),
        );

        matcher.prepare();

        assert_eq!(result, matcher.area_vec);

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
            matcher.add_black_pixel(x, 0);
        }

        assert_eq!(vec![true; 10], matcher.pixel_arr);      
    }

    #[test]
    fn test_staff_matched() {

        let width = 10;
        let pattern = vec!(-2, 3, -4, 3, -5, 3, -5, 3, -5, 3, -5, 3);

        let (pixel_arr, height) = generate_staff_image(
            pattern,
            width
        );

        let mut matcher = StaffMatcher::new(
            width,
            height
        );

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(*x, *y));
        
        let result = vec!(
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            );

        assert_eq!(result, matcher.iter().next().unwrap().areas);

    }


    #[test]
    fn test_two_staff_matched() {
        
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

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(*x, *y));
        
        let mut iter = matcher.iter();

        let staff1 = iter.next().unwrap().areas;
        let staff2 = iter.next().unwrap().areas;

        assert_eq!(vec!(
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
            Area::Spacing(5),
            Area::Line(3),
        ), staff1);

        assert_eq!(vec!(
            Area::Line(2),
            Area::Spacing(2),
            Area::Line(2),
            Area::Spacing(2),
            Area::Line(2),
            Area::Spacing(2),
            Area::Line(2),
            Area::Spacing(2),
            Area::Line(2),
        ), staff2);

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

        pixel_arr.iter().for_each(|(x, y)| matcher.add_black_pixel(*x, *y));

        assert!(matcher.iter().next().is_none());
    }

}