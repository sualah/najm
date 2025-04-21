use rand::{distributions::Alphanumeric, Rng};
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn increment_counter() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn get_counter() -> usize {
    COUNTER.load(Ordering::SeqCst)
}
pub fn generate_random_string(len: usize) -> String {
    increment_counter();
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    
    format!("{rand_string}.{}", get_counter())
}