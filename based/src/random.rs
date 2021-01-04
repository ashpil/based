use std::cell::UnsafeCell;
use std::lazy::Lazy;
use rand_xoshiro::Xoshiro256Plus;
use rand::SeedableRng;

pub fn with_rng<T>(f: impl FnOnce(&mut Xoshiro256Plus) -> T) -> T {
    #[thread_local]
    static RNG: Lazy<UnsafeCell<Xoshiro256Plus>> = Lazy::new(|| UnsafeCell::new(Xoshiro256Plus::seed_from_u64(1)));

    f(unsafe { &mut *RNG.get() })
}

