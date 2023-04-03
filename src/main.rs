struct Staff {
}
enum Zone {
    Line(usize),
    Spacing(usize),    
}

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
    fn match_staff(&self) -> Result<Staff, StaffMatchError> {
        if self.pixel_arr.len() == 0 {
            return Err(StaffMatchError::IsEmpty);
        }


        let density_arr: Vec<f32> = self.pixel_arr
            .chunks(self.width)
            .map(
                |v| {
                    v.iter().filter(|&p| *p).count() as f32
                    /
                    self.width as f32
                }
            ).collect();


        let mut result_arr:Vec<Zone> = Vec::new();
        for d in density_arr.iter() {
            let last_zone = result_arr.last_mut();
            let is_line = *d >= 0.5;
            match last_zone {
                Some(Zone::Line(ref mut line_size)) if is_line => 
                    *line_size += 1,
                Some(Zone::Line(..)) if !is_line => 
                    result_arr.push(Zone::Spacing(1)),
                Some(Zone::Spacing(ref mut spacing_size)) if !is_line => 
                    *spacing_size += 1,
                _ => result_arr.push(Zone::Line(1)),
                
            }
        };

        todo!("try generating staff");
        
    }

    fn add_black_pixel(&mut self, x: usize, y: usize) {
        if let Some(pixel) = self.pixel_arr.get_mut((y - 1) * self.height + (x - 1)) {
            *pixel = true;
        }
    }
}

#[derive(PartialEq, Debug)]
enum StaffMatchError {
    OutOfBounds,
    IsEmpty,
}


fn main() {
    println!("Hello, world!");
}


#[test]
fn test_empty_row_arr() {

    let matcher = StaffMatcher::new(0, 0);

    //assert_eq!(Err(StaffMatchError::IsEmpty), matcher.match_staff());    
}

#[test]
fn test_add_point() {

    let mut matcher = StaffMatcher::new(10, 1);

    for x in 1..11 {
        matcher.add_black_pixel(x, 1);
    }

    assert_eq!(vec![true; 10], matcher.pixel_arr);      
}

#[test]
fn test_staff_not_matched() {
    let mut matcher = StaffMatcher::new(10, 10);

}