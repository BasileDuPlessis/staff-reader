struct StaffMatcher {
    area_size: (usize, usize),
    pixel_arr: Vec<Vec<u8>>,
}

impl StaffMatcher {
    fn new(width: usize, height: usize) -> StaffMatcher {
        StaffMatcher {
            area_size: (width, height),
            pixel_arr: vec![vec![0u8; width]; height],
        }        
    }
    fn match_staff(&self) -> Result<bool, StaffMatchError> {
        if self.pixel_arr.len() == 0 {
            return Err(StaffMatchError::IsEmpty);
        }
        Ok(true)
    }
    fn add_pixel(&mut self, w: usize, h: usize) {
        if let Some(line) = self.pixel_arr.get_mut(h) {
            if let Some(column) = line.get_mut(w) {
                *column = 1;
            }
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

    assert_eq!(Err(StaffMatchError::IsEmpty), matcher.match_staff());    
}

#[test]
fn test_add_point() {

    let mut matcher = StaffMatcher::new(10, 1);

    for p in 0..10 {
        matcher.add_pixel(p, 0);
    }

    assert_eq!(vec![1; 10], matcher.pixel_arr[0]);

      
}