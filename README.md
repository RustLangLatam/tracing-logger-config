# Tracing Logger Config

`tracing_logger_config` is a Rust crate designed to simplify logging configuration and tracing setup using OpenTelemetry and the `tracing` crate. It provides an easy-to-use interface for managing log files, controlling log rotation, and setting up distributed tracing with optional Jaeger backend support.

## Features

- **Flexible Logging Configuration**: Easily configure log paths, error log paths, log rotation strategies, and logging levels.
- **Tracing Initialization**: Set up tracing with OpenTelemetry, including optional support for a Jaeger backend.
- **Log Rotation**: Supports various log rotation strategies: never, minutely, hourly, and daily.
- **Serde Integration**: Configuration structs are serializable and deserializable using Serde, making integration with configuration files straightforward.

## Installation

To use `tracing_logger_config` in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
tracing_logger_config = "0.1.0"
```

## Usage

### Basic Configuration

Here's how to configure logging and tracing in your application:

```rust
use tracing_logger_config::{Config, init_tracing, RotationKind, LevelInner, ExporterEndpoint};

fn main() -> anyhow::Result<()> {
    let config = Config {
        log_path: Some("logs/app.log".into()),
        log_error_path: Some("logs/error.log".into()),
        rotation: RotationKind::Daily,
        level: Some(LevelInner::Info),
        exporter_endpoint: Some(ExporterEndpoint {
            host: "localhost".to_string(),
            port: 6831,
        }),
    };

    let _guard = init_tracing(config.exporter_endpoint.as_ref(), Some(&config))?;

    // Your application code here

    Ok(())
}
```

### Configuration Details

The `Config` struct is used to specify logging and tracing settings. It allows you to set:

- Paths for log files and error logs
- Log rotation strategies (e.g., never, minutely, hourly, daily)
- Logging levels (e.g., trace, debug, info, warn, error)
- Optional endpoint for exporting tracing data

### Tracing Initialization

The `init_tracing` function sets up tracing with OpenTelemetry, including optional Jaeger backend support:

```rust
use tracing_logger_config::{init_tracing, Config, ExporterEndpoint};

fn setup_tracing() -> anyhow::Result<()> {
    let exporter_endpoint = ExporterEndpoint {
        host: "localhost".to_string(),
        port: 6831,
    };

    let config = Config {
        log_path: Some("logs/app.log".into()),
        level: Some(LevelInner::Debug),
        ..Default::default()
    };

    let _guard = init_tracing(Some(&exporter_endpoint), Some(&config))?;

    // Tracing is now initialized

    Ok(())
}
```

## License

This crate is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request to contribute to `tracing_logger_config`.

For more information, see the [CONTRIBUTING](CONTRIBUTING.md) guidelines.
```