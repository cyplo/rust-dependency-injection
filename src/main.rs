use std::sync::Arc;
use std::time::Instant;

trait Clock {
    fn now(&self) -> Instant;
}

struct TimestampingRepository<ClockType> {
    clock: Arc<ClockType>,
    storage: Vec<Instant>,
}

impl<ClockType> TimestampingRepository<ClockType>
where
    ClockType: Clock,
{
    // gets an Arc as the clock can change its state independently (can tick in parallel to your code)
    fn with_clock(clock: Arc<ClockType>) -> Self {
        TimestampingRepository {
            clock,
            storage: vec![],
        }
    }

    fn store(&mut self) {
        self.storage.push(self.clock.now());
    }

    fn all_stored(&self) -> Vec<Instant> {
        self.storage.clone()
    }
}

struct SystemClock;

impl SystemClock {
    fn new() -> Arc<Self> {
        Arc::new(SystemClock {})
    }
}

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

fn main() {
    let clock = SystemClock::new();
    let mut repository = TimestampingRepository::with_clock(clock);
    repository.store();
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
        let consumable_clock_handle = FakeClock::with_time(Instant::now());
        let movable_clock_handle = consumable_clock_handle.clone();
        let mut repository = TimestampingRepository::with_clock(consumable_clock_handle);

        repository.store();
        movable_clock_handle.move_by(Duration::from_secs(32));
        repository.store();

        let stored_values = repository.all_stored();
        let time_difference = stored_values[1] - stored_values[0];

        assert_eq!(32, time_difference.as_secs());
    }

    struct FakeClock {
        now: Instant,
        move_by_secs: AtomicUsize,
    }

    impl FakeClock {
        fn with_time(now: Instant) -> Arc<Self> {
            Arc::new(FakeClock {
                now,
                move_by_secs: AtomicUsize::new(0),
            })
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
