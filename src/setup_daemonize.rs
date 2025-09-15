use daemonize::Daemonize;

pub fn config(daemonize: bool) {
    if !daemonize {
        return;
    }

    let daemonize = Daemonize::new();
    match daemonize.start() {
        Ok(_) => tracing::info!("Daemonized successfully"),
        Err(e) => tracing::error!("Error while daemonize: {}", e),
    }
}
