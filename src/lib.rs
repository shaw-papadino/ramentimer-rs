pub struct Clock {
    seconds: u64,
}

impl Clock {
    pub fn get_minutes(&self) -> u64 {
        self.seconds / 60
    }

    pub fn get_seconds(&self) -> u64 {
        self.seconds
    }

    pub fn new(minutes: u64) -> Clock {
        Clock {
            seconds: minutes * 60,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_set_seconds() {
        let clock = Clock::new(1);
        assert_eq!(clock.get_seconds(), 60);
    }
}
