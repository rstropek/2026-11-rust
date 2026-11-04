mod wall_clock {
    #[derive(Copy, Clone)]
    pub struct WallClock {
        hours: u8,
        minutes: u8,
    }

    impl WallClock {
        pub fn new(hours: u8, minutes: u8) -> Self {
            Self { hours, minutes }
        }

        pub fn hours(&self) -> u8 {
            self.hours
        }

        pub fn minutes(&self) -> u8 {
            self.minutes
        }

        pub fn add_minutes(&mut self, minutes_to_add: u8) {
            let total_minutes = self.minutes as u16 + minutes_to_add as u16;
            self.hours = ((self.hours as u16 + total_minutes / 60) % 24) as u8;
            self.minutes = (total_minutes % 60) as u8;
        }

        pub fn add_minutes_immutable(&self, minutes_to_add: u8) -> Self {
            let total_minutes = self.minutes as u16 + minutes_to_add as u16;
            let hours = ((self.hours as u16 + total_minutes / 60) % 24) as u8;
            let minutes = (total_minutes % 60) as u8;
            Self { hours, minutes }
        }
    }
}

fn main() {
    let mut clock = wall_clock::WallClock::new(23, 30);
    clock.add_minutes(45);
    println!("Clock: {:02}:{:02}", clock.hours(), clock.minutes());

    let new_clock = clock;
    println!("Clock: {:02}:{:02}", new_clock.hours(), new_clock.minutes());

    let clock = wall_clock::WallClock::new(10, 30);
    let clock = clock.add_minutes_immutable(45);
    println!("Clock: {:02}:{:02}", clock.hours(), clock.minutes());
}
