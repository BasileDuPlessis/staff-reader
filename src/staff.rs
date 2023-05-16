
#[derive(Clone, Debug, PartialEq)]
pub enum Area {
    Line(usize),
    Spacing(usize),    
}

#[derive(Debug, PartialEq)]
pub enum StaffError {
    NoMatch,
    StaffComplete,
    StaffStartOnlyWithLine
}

#[derive(Debug, Clone)]
pub struct Staff {
    areas: Vec<Area>,
    pub complete: bool,
}


impl Staff {
    pub fn new(area: &Area) -> Result<Staff, StaffError> {
        match area {
            &Area::Line(_) => Ok(Staff {
                areas: vec![area.clone()],
                complete: false,
            }),
           _ => Err(StaffError::StaffStartOnlyWithLine)
        }
        
    }

    fn check_area_size(&self, new: usize, first: usize) -> bool {

        let max = (first as f32 * 1.2) as usize;
        let min  = (first as f32 * 0.8) as usize;

        new >= min && new <= max

    }

    pub fn add_area(&mut self, area_to_match: &Area) -> Result<(), StaffError> {

        if self.complete {
            return Err(StaffError::StaffComplete);
        }
 
        let area_slice = self.areas.as_slice();

        match area_to_match {
            &Area::Line(new)=> {
                match area_slice {
                    [Area::Line(first), .., Area::Spacing(_)] 
                        if self.check_area_size(new, *first) =>
                            self.areas.push(area_to_match.clone()),
                    _ => return Err(StaffError::NoMatch),
                }
            }
            &Area::Spacing(new) => {
                match area_slice {
                    [Area::Line(_)] => self.areas.push(area_to_match.clone()),
                    [Area::Line(_), Area::Spacing(first), .., Area::Line(_)] 
                        if self.check_area_size(new, *first) =>
                            self.areas.push(area_to_match.clone()),
                    _ => return Err(StaffError::NoMatch),
                }
            }
        }

        if self.areas.len() == 9 {
            self.complete = true;
        }

        Ok(())

    }
}



#[cfg(test)]
mod tests {
    use crate::staff::StaffError;

    use super::{Staff, Area};

    #[test]
    fn test_area_no_match() {

        let mut staff = Staff::new(&Area::Line(5)).unwrap();

        assert_eq!(Err(StaffError::NoMatch), staff.add_area(&Area::Line(7)));

        let mut staff = Staff::new(&Area::Line(5)).unwrap();
        staff.add_area(&Area::Spacing(10));

        assert_eq!(Err(StaffError::NoMatch), staff.add_area(&Area::Spacing(10)));

        assert!(Staff::new(&Area::Spacing(5)).is_err());

    }

    #[test]
    fn test_staff_complete() {

        let mut staff = Staff::new(&Area::Line(5)).unwrap();
        assert!(!staff.complete);
        staff.add_area(&Area::Spacing(10));
        assert!(!staff.complete);
        staff.add_area(&Area::Line(5));
        assert!(!staff.complete);
        staff.add_area(&Area::Spacing(10));
        assert!(!staff.complete);
        staff.add_area(&Area::Line(5));
        assert!(!staff.complete);
        staff.add_area(&Area::Spacing(10));
        assert!(!staff.complete);
        staff.add_area(&Area::Line(5));
        assert!(!staff.complete);
        staff.add_area(&Area::Spacing(10));
        assert!(!staff.complete);
        staff.add_area(&Area::Line(5));
        assert!(staff.complete);

    }


    #[test]
    fn test_check_area_in_deviation_eighty_pourcent() {
        
        let mut staff = Staff {
            areas: vec![Area::Line(5), Area::Spacing(40), Area::Line(5), Area::Spacing(40)],
            complete: false
        };
        assert!(staff.add_area(&Area::Line(6)).is_ok());
        assert!(staff.add_area(&Area::Spacing(45)).is_ok());

    }

    #[test]
    fn test_check_area_outside_standard_deviation() {
        
        let mut staff = Staff {
            areas: vec![Area::Line(5), Area::Spacing(40), Area::Line(5), Area::Spacing(40)],
            complete: false
        };
        assert!(staff.add_area(&Area::Line(7)).is_err());
        assert!(staff.add_area(&Area::Line(3)).is_err());

        staff.add_area(&Area::Line(5));

        assert!(staff.add_area(&Area::Spacing(50)).is_err());
        assert!(staff.add_area(&Area::Spacing(31)).is_err());

    }
}