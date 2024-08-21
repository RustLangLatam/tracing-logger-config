use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use std::{env, str::FromStr};
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{fmt,
                         fmt::{format::FmtSpan, time::ChronoLocal, writer::BoxMakeWriter},
                         prelude::*};

use crate::{config::{Config, LogPath, RotationKind},
            ExporterEndpoint};

/// Initializes an OpenTelemetry tracing subscriber with a Jaeger backend.
pub fn init_tracing(
    exporter_endpoint: Option<&ExporterEndpoint>,
    config: Option<&Config>,
) -> anyhow::Result<WorkerGuard> {
    unsafe {
        env::set_var("OTEL_BSP_MAX_EXPORT_BATCH_SIZE", "12");
    }

    let mut env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or(tracing_subscriber::EnvFilter::from("info"));

    let (writer, guard) = if let Some(ref c) = config {
        // Log all `tracing` events to files prefixed with `debug`. Since these
        if let Some(level) = c.level {
            env_filter = tracing_subscriber::EnvFilter::try_new(level.to_string())
                .unwrap_or(tracing_subscriber::EnvFilter::from("info"));
        }

        // Log all `tracing` events to files prefixed with `debug`. Since these
        // files will be written to very frequently, roll the log file every hourly.
        let log_path = c.log_path();

        if let Some(log_path) = log_path {
            let log_file = match c.rotation {
                RotationKind::Never =>
                    rolling::never(log_path.directory.as_str(), log_path.filename.as_str()),
                RotationKind::Minutely =>
                    rolling::minutely(log_path.directory.as_str(), log_path.filename.as_str()),
                RotationKind::Hourly =>
                    rolling::hourly(log_path.directory.as_str(), log_path.filename.as_str()),
                RotationKind::Daily =>
                    rolling::daily(log_path.directory.as_str(), log_path.filename.as_str()),
            };

            let (log_non_blocking, guard) = tracing_appender::non_blocking(log_file);

            let error_log_file = c.log_error_path().unwrap_or({
                let filename = format!("error_{}", log_path.filename.as_str());
                LogPath { filename, directory: log_path.directory.clone() }
            });

            let level: tracing::Level = tracing::Level::from_str(env_filter.to_string().as_str())?;

            let error_file = rolling::daily(error_log_file.directory, error_log_file.filename)
                .with_max_level(tracing::Level::WARN);

            (
                BoxMakeWriter::new(
                    log_non_blocking
                        .with_max_level(level)
                        .with_min_level(tracing::Level::INFO)
                        .and(error_file),
                ),
                guard,
            )
        } else {
            let (log_non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
            (BoxMakeWriter::new(log_non_blocking), guard)
        }
    } else {
        let (log_non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
        (BoxMakeWriter::new(log_non_blocking), guard)
    };

    // | `%F`  | `2001-07-08`   | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.
    // | `%X`  | `00:34:60`     | Locale's time representation (e.g., 23:13:48).
    // | `%.3f`| `.026`         | Decimal fraction of a second with a fixed length of 3.
    // | `%:z` | `+09:30`       | Offset from the local time to UTC (with UTC being `+00:00`).
    let timer = ChronoLocal::new("[%F %X%.3f %:z]".into());

    let collector = tracing_subscriber::registry().with(env_filter).with(
        fmt::layer()
            .with_timer(timer)
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_writer(writer),
    );

    if let Some(endpoint) = exporter_endpoint {
        let otlp_exporter =
            opentelemetry_otlp::new_exporter().tonic().with_endpoint(endpoint.get_host());

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter)
            .install_batch(opentelemetry_sdk::runtime::Tokio)?
            .tracer("trace_app");

        // Create a layer with the configured tracer
        let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

        collector.with(otel_layer).try_init()?;
    } else {
        collector.try_init()?;
    }

    Ok(guard)
}
