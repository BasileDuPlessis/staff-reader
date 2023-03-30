struct StaffMatcher {
    density_arr: Vec<u8>,
}

impl StaffMatcher {
    fn new() -> StaffMatcher {
        StaffMatcher {
            density_arr : vec![],
        }
    }
    fn match_staff(&self) -> Result<bool, StaffMatchError> {
        if self.density_arr.len() == 0 {
            return Err(StaffMatchError::IsEmpty);
        }
        Ok(true)
    }
    fn receive_point(x: u16, y: u16) {
        todo!("Add code to compute density per continuous line")
    }
}

#[derive(PartialEq, Debug)]
enum StaffMatchError {
    WrongLineNumber,
    IsEmpty,
}


fn main() {
    println!("Hello, world!");
}


#[test]
fn empty_row_arr() {

    let matcher = StaffMatcher::new();

    assert_eq!(Err(StaffMatchError::IsEmpty), matcher.match_staff());    
}