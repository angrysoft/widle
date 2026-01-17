mod watcher;
use watcher::Watcher;
fn main() {
    let mut watcher = Watcher::new(5000);
    watcher.run();
}

// --- Dispatch Implementations ---
