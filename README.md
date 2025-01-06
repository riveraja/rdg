# Rust Data Generator for Redis or Valkey

Generates fake data for training purposes only.

## Installation

Download the release binary applicable for your operating system.

```bash
curl -LO https://github.com/riveraja/rdg/releases/download/v0.5.0/rs-data-generator-aarch64_apple_darwin
```

```bash
curl -LO https://github.com/riveraja/rdg/releases/download/v0.5.0/rs-data-generator-x86_64-apple-darwin
```

```bash
curl -LO https://github.com/riveraja/rdg/releases/download/v0.5.0/rs-data-generator-x86_64-unknown-linux-gnu
```

Make the binary executable.

```bash
chmod +x ./rs-data-generator*
```

Use the help option

```bash
./rs-data-generator-aarch64_apple_darwin -h
Usage: rs-data-generator-aarch64_apple_darwin [OPTIONS]

rs-data-generator: A data generator written in Rust for Redis/Valkey

v0.5.0

Options:
  -u, --uri      The Redis connection string. (default: redis://127.0.0.1:6379)
  -c, --count    Total count of records to be generated. (default: 1000)
  -t, --types    Types of commands to execute. (default: [])
  -b, --batch    Pipeline size. (default: 50)
  -h, --help     Show this help message.
```

## Generate fake data

Specify each command method separately.

```bash
./rs-data-generator-aarch64_apple_darwin -t json -t hset -t set -c 10000 -b 1000
Generating data
10000 SET
10000 HSET
10000 JSON.SET
Elapsed time 421.505375ms
```
