use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(non_snake_case)]
mod InitState {
    #[repr(u8)]
    enum States {
        Uninit,
        Initializing,
        Init,
    }

    pub(super) type Value = std::sync::atomic::AtomicU8;
    pub(super) const UNINIT: u8 = States::Uninit as _;
    pub(super) const INITIALIZING: u8 = States::Initializing as _;
    pub(super) const INIT: u8 = States::Init as _;
}

pub enum NatStoreError {
    Concurrent,
    AlreadyStored(usize),
}

pub struct NatHolder {
    init_state: InitState::Value,
    value: AtomicUsize,
}
impl NatHolder {
    pub const fn new() -> Self {
        Self {
            init_state: InitState::Value::new(InitState::UNINIT),
            value: AtomicUsize::new(0),
        }
    }
    pub fn store(&self, value: usize) -> Result<(), NatStoreError> {
        if self.init_state.compare_and_swap(
            InitState::UNINIT,
            InitState::INITIALIZING,
            Ordering::Acquire,
        ) != InitState::UNINIT
        {
            return self.read().map_or(Err(NatStoreError::Concurrent), |cur| {
                if cur == value {
                    Ok(())
                } else {
                    Err(NatStoreError::AlreadyStored(cur))
                }
            });
        }
        self.value.store(value, Ordering::Relaxed);
        self.init_state.store(InitState::INIT, Ordering::Release);
        Ok(())
    }
    pub fn read(&self) -> Option<usize> {
        if self.init_state.load(Ordering::Acquire) == InitState::INIT {
            Some(self.value.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}

// This is actually safe, since we are guarding Cell with atomics.
unsafe impl Sync for NatHolder {}
