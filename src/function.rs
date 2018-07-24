use std::time::Duration;
use std::time::Instant;

trait Clock {
    fn now(&self) -> Instant;
}

struct FakeClock {
    now: Instant,
}

impl FakeClock {
    fn with_time(now: Instant) -> Self {
        FakeClock { now }
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        self.now
    }
}

fn format_time_difference(clock1: impl Clock, clock2: impl Clock) -> String {
    "".to_string()
}

#[cfg(test)]
mod should {

    use super::*;

    #[test]
    fn handle_seconds() {
        let base_time = Instant::now();
        let clock1 = FakeClock::with_time(base_time);
        let clock2 = FakeClock::with_time(base_time + Duration::from_secs(32));

        let formatted_time_difference = format_time_difference(clock1, clock2);

        assert_eq!("32 seconds ago", formatted_time_difference);
    }

}
