use std::sync::Arc;
use std::time::Instant;

trait Clock {
    fn now(&self) -> Instant;
}

struct TimestampingRepository<ClockType> {
    clock: Arc<ClockType>,
}

impl<ClockType> TimestampingRepository<ClockType>
where
    ClockType: Clock,
{
    fn with_clock(clock: Arc<ClockType>) -> Self {
        TimestampingRepository { clock }
    }

    fn store(&mut self) {
        unimplemented!()
    }

    fn all_stored(&self) -> Vec<Instant> {
        unimplemented!()
    }
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
        move_by_millis: AtomicUsize,
    }

    impl FakeClock {
        fn with_time(now: Instant) -> Arc<Self> {
            Arc::new(FakeClock {
                now,
                move_by_millis: AtomicUsize::new(0),
            })
        }

        // WAT no `mut`
        fn move_by(&self, duration: Duration) {
            self.move_by_millis
                .store(duration.as_secs() as usize, Ordering::SeqCst);
        }
    }

    impl Clock for FakeClock {
        fn now(&self) -> Instant {
            unimplemented!()
        }
    }

}
