# medea

medea is a command-line developers toolbox, written in Rust. Similar to projects like [CyberChef](https://github.com/gchq/CyberChef) and [DevToys](https://github.com/veler/DevToys), it offers tools for quick text generation and processing, like creating hashes and parsing jwts. Being a command line application, medea is easy to install and offers extension through pipe chaining and bash scripting.

## Features

- Text Generation
  - UUID
- Text Processing
  - Hash generation
  - Timestamp conversion

## Installation

#### TODO: Installation script

You can grab the binary from the latest release here: https://github.com/haondt/medea/releases

Alternatively, you can clone the repository and build with Cargo.

## Usage

The basic usage is `medea [command] --options`. See `medea help` or `medea help [command] help` for more details. Here are some example usages:

```shell
# generate an HS256 hash
medea hash -a sha256 --hmac 'my secret' 'my data'

# generate some uuids
medea uuid -uc 5

# convert timestamps
medea ts --format iso -z America/Los_Angeles 1678742400
```

## Tests

Run tests with

```shell
cargo test
```

## License

See [LICENSE](./LICENSE)


