## Testing clock-reliant behaviour or how to use internal mutability in Rust

Hello and welcome to the newest episode on testing in Rust.  
Imagine you want to write a timestamping repository of some sorts, that will associate the timestamp of when the storage operation was invoked with the stored value.
How to write it in Rust ? And more importantly - how to test it ?
I would like to share a solution I found and talk a bit about how it works.

Please note that this solution can be used anywhere where you need to pass a handle that is remembered by the production code, and that thing it points to you then want to change from the test.
 
```rust
trait Clock {
    fn now(&self) -> Instant;
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

#[cfg(test)]
mod should {

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
            let move_by_secs = self.move_by_secs.load(Ordering::SeqCst) as u64;
            self.now + Duration::from_secs(move_by_secs)
        }
    }

}
```

That's a lot of code. And I already skipped `use`s and some definitions to make it less.  
If you want to get the full source code that to follow along - try this [playground](https://play.rust-lang.org/?gist=7f47c441732b543a918cb491487196d4&version=stable&mode=debug&edition=2015) or [this repo](https://github.com/cyplo/rust-dependency-injection) for the full project including production code usage.  

Let's start with the test itself.  
The clock appears to be immutable (immovable) in the test, yet we call `move_by` on it and the whole thing appears to be working somehow.
First question: can't we just make the clock mutable and skip all this ?
It appears that sadly (but fortunately) Rust prevents us from doing so.
We [cannot](https://doc.rust-lang.org/book/second-edition/ch04-02-references-and-borrowing.html) both have a immutable and mutable borrow of the clock in the same scope.
For the full example with an error go [here](https://play.rust-lang.org/?gist=3e496f857f1e016c596ec7c4060538df&version=stable&mode=debug&edition=2015).  

What is this sorcery then ?  
We use a type that provides [`Interior Mutability`](https://doc.rust-lang.org/book/second-edition/ch15-05-interior-mutability.html), namely [`AtomicUsize`](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/std/sync/atomic/struct.AtomicUsize.html).  
On the outside - it look immutable, yet it provides a thread-safe and very narrow method of mutating the underlying state. Rust compiler is happy and our test is happy.  

I wouldn't use this as a pattern in production code - the borrow checker rules are there for a reason.  
Please treat it as an escape hatch to be used in specific situations, situations like this.  

Happy Rusting !


