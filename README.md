# medea

medea is a command-line developers toolbox, written in Rust. Similar to projects like [CyberChef](https://github.com/gchq/CyberChef) and [DevToys](https://github.com/veler/DevToys), it offers tools for quick text generation and processing, like creating hashes and parsing jwts. Being a command line application, medea is easy to install and offers extension through pipe chaining and bash scripting.

## Features

- Text Generation
  - UUID generation
  - Random data generation
- Text Processing
  - Hash generation
  - Timestamp conversion
- Encoding and Decoding
  - Base conversion
  - JWT parsing and creation

## Installation


#### Option 1: Download binary

You can grab the binary from the latest release: https://github.com/haondt/medea/releases

#### Option 2: Install with Cargo

```shell
cargo install haondt-medea
```

#### Option 3: Install manually

TODO: Installation script

## Usage

The basic usage is `medea [command] <options>`. See `medea help` or `medea help [command]` for more details. Here are some example usages:

```shell
# generate an HS256 hash
medea hash -a sha256 --hmac 'my secret' 'my data'

# generate some uuids
medea uuid -u 5

# convert timestamps
medea ts --to iso -z America/Los_Angeles 1678742400

# generate random data
medea rnd -t hex 16
```

## Tests

Run tests with

```shell
cargo test
```

## License

See [LICENSE](./LICENSE)


