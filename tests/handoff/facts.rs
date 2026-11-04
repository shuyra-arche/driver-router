use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock};

use env::{Mark, PieToken, TaskId};

#[derive(Clone, Copy)]
pub(crate) struct Facts {
    pub(crate) alive: bool,
    pub(crate) vestor: TaskId,
    pub(crate) owner: TaskId,
    pub(crate) mark: Mark,
}

static FACTS: OnceLock<Mutex<HashMap<usize, Facts>>> = OnceLock::new();
pub(crate) static ACCEPTED: AtomicUsize = AtomicUsize::new(usize::MAX);
static TEST_LOCK: Mutex<()> = Mutex::new(());

pub(crate) fn facts() -> &'static Mutex<HashMap<usize, Facts>> {
    FACTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn reset() -> MutexGuard<'static, ()> {
    let guard = TEST_LOCK.lock().unwrap();
    facts().lock().unwrap().clear();
    ACCEPTED.store(usize::MAX, Ordering::SeqCst);
    guard
}

pub(crate) fn token(value: u64) -> PieToken {
    PieToken::from_bytes(&value.to_le_bytes()).unwrap()
}

pub(crate) fn register(token: PieToken, alive: bool, vestor: TaskId, owner: TaskId, mark: Mark) {
    facts().lock().unwrap().insert(
        token.get(),
        Facts {
            alive,
            vestor,
            owner,
            mark,
        },
    );
}
