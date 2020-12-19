use std::fmt;

pub fn trial(num: i32)-> (i32, i32){
    let hours: i32 = num /60;
    let min: i32 = (60 + (num % 60))%60;
    (hours, min)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    minutes: i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: (((60 * hours + minutes) % 1440) + 1440)%1440
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes/60, self.minutes%60)
    }
}

