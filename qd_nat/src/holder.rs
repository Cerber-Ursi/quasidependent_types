use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum NatStoreError {
    #[error("Attempted to concurrently create multiple instances of N: Nat")]
    Concurrent,
    #[error("Attempted to override already stored value {0} with {1}")]
    AlreadyStored(usize, usize),
    #[error("Attempted to create composite number {0} before its components")]
    UnknownCompositeParts(&'static str),
}

pub struct NatHolder {
    init_state: InitState::Value,
    value: AtomicUsize,
}
impl NatHolder {
    pub const fn new() -> Self {
        Self {
            init_state: InitState::Value::new(InitState::UNINIT),
            // We can store any value here, since we can't read it anyway, while init_state is not INIT
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
                    Err(NatStoreError::AlreadyStored(cur, value))
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
