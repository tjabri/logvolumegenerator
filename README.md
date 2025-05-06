# Log Volume Generator

A utility for generating log output at a specified volume over a given duration. This tool is useful for testing log ingestion systems, monitoring solutions, or benchmarking log processing pipelines.

## Features

- Generate logs at a specified volume rate (e.g., 10MB/hour)
- Control the duration of log generation
- Limit the total number of log lines output
- Choose between plaintext or JSON log formats
- Configurable via environment variables or configuration file

## Installation

### Using Docker

The easiest way to use Log Volume Generator is via the published Docker container:

```bash
docker pull ghcr.io/tjabri/logvolumegenerator:latest
```

You can also use a specific version by using the git SHA as a tag:

```bash
docker pull ghcr.io/tjabri/logvolumegenerator:abc123
```

Replace `abc123` with the actual git SHA of the version you want to use.

### Building from Source

If you prefer to build from source, you'll need Rust installed:

```bash
git clone https://github.com/tjabri/logvolumegenerator.git
cd logvolumegenerator
cargo build --release
```

The binary will be available at `target/release/logvolumegenerator`.

## Usage

### Basic Usage

Generate 10MB of logs over 1 hour:

```bash
LOG_output_size=10MB LOG_duration=1h logvolumegenerator
```

### Docker Usage

```bash
docker run -e LOG_output_size=10MB -e LOG_duration=1h ghcr.io/tjabri/logvolumegenerator:latest
```

### Configuration Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `output_size` | Target volume of logs to generate | Required | `10MB`, `1GB` |
| `duration` | Duration over which to generate logs | Required | `1h`, `30m`, `1d` |
| `max_lines_output` | Maximum number of log lines to output (0 = unlimited) | 0 | `1000` |
| `output_format` | Format of log output (plaintext or json) | plaintext | `json` |

### Environment Variables

All configuration options can also be set using environment variables with the `LOG_` prefix:

```bash
LOG_output_size=10MB LOG_duration=1h LOG_output_format=json logvolumegenerator
```

With Docker:

```bash
docker run -e LOG_output_size=10MB -e LOG_duration=1h -e LOG_output_format=json ghcr.io/tjabri/logvolumegenerator:latest
```

### Configuration File

You can also use a configuration file named `configuration.toml` in the current directory:

```toml
output_size = "10MB"
duration = "1h"
max_lines_output = 1000
output_format = "json"
```

## Examples

### Generate 100MB of plaintext logs over 30 minutes

```bash
LOG_output_size=100MB LOG_duration=30m logvolumegenerator
```

### Generate 1GB of JSON logs over 2 hours

```bash
LOG_output_size=1GB LOG_duration=2h LOG_output_format=json logvolumegenerator
```

### Generate logs at 5MB/hour for 4 hours, but limit to 10,000 lines

```bash
LOG_output_size=20MB LOG_duration=4h LOG_max_lines_output=10000 logvolumegenerator 
```

### Using Docker to generate logs and pipe to a file

```bash
docker run -e LOG_output_size=50MB -e LOG_duration=1h ghcr.io/tjabri/logvolumegenerator:latest > logs.txt
```

## Use Cases

- Testing log ingestion systems
- Benchmarking log processing pipelines
- Simulating application log output for monitoring system testing
- Stress testing log storage systems

## Personal Notes

This is my first Rust project, so please don't judge me too harshly. I'm still learning the language and trying to get a
feel for the idioms.
