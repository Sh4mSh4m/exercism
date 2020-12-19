use std::fmt;

pub fn trial(num: i32)-> (i32, i32){
    let hours: i32 = num /60;
    let min: i32 = (60 + (num % 60))%60;
    (hours, min)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}


impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_set: i32;
        let mins_set: i32;
        let mut hour_tmp:i32;


        mins_set = (60 + (&minutes % 60))%60;
        if minutes < 0 {
            hour_tmp = hours + (minutes /60)-1;
        } else {
            hour_tmp = hours + (minutes /60);
        }

        hours_set = (24 + (hour_tmp % 24))%24;
        Self {
            hours:hours_set,
            minutes: mins_set
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mins_set = (60 + ((&self.minutes + &minutes)%60))%60;
        let hour_tmp:i32;
        if minutes < 0 {
            hour_tmp = ((&self.minutes+minutes) /60)-1;
        } else { 
            hour_tmp = ((&self.minutes+minutes) /60);
        }

        let hours_set = ((24 + ((self.hours + hour_tmp)%24))%24);
        Self {
            hours: hours_set,
            minutes: mins_set,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

