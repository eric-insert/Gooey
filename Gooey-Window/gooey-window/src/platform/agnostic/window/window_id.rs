use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowID(pub usize);

static WINDOW_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl WindowID {
    pub fn generate() -> Self {
        WindowID(WINDOW_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}