use tracing::Level;
use tracing_subscriber::{EnvFilter, prelude::*};

pub fn config(verbose: bool) {
    // 로거 설정
    tracing_subscriber::registry()
        // 로거 기본 설정
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_target(verbose)
                .with_line_number(verbose),
        )
        // 환경변수 불러오기
        .with(
            EnvFilter::builder()
                .with_default_directive(if verbose { Level::DEBUG } else { Level::INFO }.into())
                .from_env_lossy(),
        )
        .init();
    tracing::debug!("Logger configured");
}
