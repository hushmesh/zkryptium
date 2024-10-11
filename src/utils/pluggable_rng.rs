#[cfg(not(feature = "std"))]
use core::panic;

use alloc::boxed::Box;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use rand::RngCore;
use spin::rwlock::RwLock;

pub trait PluggableRng: Send + Sync {
    fn new_rng(&self) -> Box<dyn RngCore>;
}

struct RngManagerInternal {
    rng: Option<Box<dyn PluggableRng>>,
}

#[derive(Clone)]
pub struct RngManager {
    state: Arc<RwLock<RngManagerInternal>>,
}

lazy_static! {
    static ref RNG_MANAGER: RngManager = RngManager::new();
}

impl RngManager {
    fn new() -> Self {
        let mgr = Self {
            state: Arc::new(RwLock::new(RngManagerInternal { rng: None })),
        };
        mgr
    }
}

pub fn set_pluggable_rng_maker(rng: Box<dyn PluggableRng>) {
    let mut state = RNG_MANAGER.state.write();
    state.rng = Some(rng);
}

pub fn get_new_rng() -> Box<dyn RngCore> {
    let rng_maker = RNG_MANAGER.state.read();
    let maker = rng_maker.rng.as_ref();
    if let Some(maker) = maker {
        return maker.new_rng();
    }
    #[cfg(feature = "std")]
    return Box::new(rand::thread_rng());
    #[cfg(not(feature = "std"))]
    panic!("No pluggable RNG maker set");
}
