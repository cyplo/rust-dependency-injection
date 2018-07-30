use std::time::Instant;

trait Clock {
    fn now(&self) -> Instant;
}

struct TimestampingRepository<'a, ClockType>
where
    ClockType: Clock + 'a,
{
    clock: &'a ClockType,
    storage: Vec<(Instant, u32)>, // (timestamp, value)
}

impl<'a, ClockType> TimestampingRepository<'a, ClockType>
where
    ClockType: Clock + 'a,
{
    fn with_clock(clock: &'a ClockType) -> Self {
        TimestampingRepository {
            clock,
            storage: vec![],
        }
    }

    fn store(&mut self, value: u32) {
        self.storage.push((self.clock.now(), value));
    }

    fn all_stored(&self) -> Vec<(Instant, u32)> {
        self.storage.clone()
    }
}

struct SystemClock;

impl SystemClock {
    fn new() -> Self {
        SystemClock {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

fn main() {
    let clock = SystemClock::new();
    let mut repository = TimestampingRepository::with_clock(&clock);

    repository.store(1);
    repository.store(2);

    println!("{:?}", repository.all_stored());
}

#[cfg(test)]
mod should {

    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::time::Duration;

    #[test]
    fn handle_seconds() {
        let clock = FakeClock::with_time(Instant::now());
        let mut repository = TimestampingRepository::with_clock(&clock);

        repository.store(1);
        clock.move_by(Duration::from_secs(32));
        repository.store(2);

        let time_difference = time_difference_between_two_stored(repository);

        assert_eq!(32, time_difference.as_secs());
    }

    fn time_difference_between_two_stored<ClockType>(
        repository: TimestampingRepository<ClockType>,
    ) -> Duration
    where
        ClockType: Clock,
    {
        let stored_values = repository.all_stored();
        let first_timestamp = stored_values[0].0;
        let second_timestamp = stored_values[1].0;
        second_timestamp - first_timestamp
    }

    struct FakeClock {
        now: Instant,
        move_by_secs: AtomicUsize,
    }

    impl FakeClock {
        fn with_time(now: Instant) -> Self {
            FakeClock {
                now,
                move_by_secs: AtomicUsize::new(0),
            }
        }

        // WAT no `mut`
        fn move_by(&self, duration: Duration) {
            self.move_by_secs
                .store(duration.as_secs() as usize, Ordering::SeqCst);
        }
    }

    impl Clock for FakeClock {
        fn now(&self) -> Instant {
            let move_by_millis = self.move_by_secs.load(Ordering::SeqCst) as u64;
            self.now + Duration::from_secs(move_by_millis)
        }
    }

}
