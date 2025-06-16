use std::thread;

use log::{debug, info, warn};
use tracing::instrument;

fn main() {
    tracing_subscriber::fmt::init();
    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || fake_database_operation(i));
        }
    });
}

#[instrument]
fn fake_database_operation(account_number: i32) {
    let start_time = std::time::Instant::now();
    info!("Reading account record from database for account number {account_number}");

    // randomly sleep up to 0.5 seconds
    let sleep_duration = std::time::Duration::from_millis((rand::random::<f64>() * 500.0) as u64);
    thread::sleep(sleep_duration);

    let elapsed_time = start_time.elapsed().as_millis();
    if sleep_duration.as_millis() > 400 {
        warn!(
            "Timeout reading account record from database for account number {account_number}; operation took {elapsed_time} ms"
        );
        return;
    } else {
        info!(
            "Successfully read account record from database for account number {account_number}; operation took {elapsed_time} ms"
        );
    }

    info!("Updating account record in database for account number {account_number}");

    // fail 50% of the time
    if rand::random::<f64>() > 0.5 {
        warn!(
            "Failed to update account record in database for account number {account_number}; record locked"
        );
    } else {
        info!(
            "Successfully updated account record in database for account number {account_number}"
        );
    }

    let elapsed_time = start_time.elapsed().as_millis();
    debug!("fake_database_operation completed successfully in {elapsed_time} ms")
}
