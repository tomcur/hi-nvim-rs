use std::time::SystemTime;

/// Prints the number of seconds since the unix epoch.
fn main() {
    let secs_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!(
        "1970-01-01 00:00:00 UTC was \
        {secs_since_epoch} seconds ago!");
}
