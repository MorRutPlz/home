use log::{debug, error, info};

#[cfg(debug_assertions)]
pub fn _debug<T: Into<String>>(message: T) {
    let message = message.into();

    debug!(
        target: "home",
        "{}",
        message,
    );
}

pub fn error<T: Into<String>>(message: T) {
    let message = message.into();

    error!(
        target: "home",
        "{}",
        message,
    );
}

pub fn info<T: Into<String>>(message: T) {
    let message = message.into();

    info!(
        target: "home",
        "{}",
        message,
    );
}
