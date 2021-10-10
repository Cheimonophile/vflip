use std::sync;

// acquires a mutex guard in a single line
pub fn lock<T>(mutex: &sync::Mutex<T>) -> sync::MutexGuard<T> {
  match mutex.lock() {
    Ok(guard) => guard,
    Err(error) => error.into_inner()
  }
}