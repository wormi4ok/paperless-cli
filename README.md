# Paperless CLI

This is a simple CLI tool to interact with [paperless-ngx](https://docs.paperless-ngx.com)

## Features

Currently this tool does only one thing: uploads documents to Paperless.

## Installation

To install `paperless-cli`, you need to have [Rust](https://www.rust-lang.org/) installed. You can then build the project using `cargo`:

```sh
git clone https://github.com/wormi4ok/paperless-cli.git
cd paperless-cli
cargo build --release
```

The compiled binary will be located in the `target/release` directory. You can move this binary to a location in your `PATH` to use it globally.

## Usage

Before using `paperless-cli`, you need to set the `PAPERLESS_API_TOKEN` environment variable with your Paperless API token and and `PAPERLESS_URL` pointing to your instance of Paperless-ngx.
If `PAPERLESS_URL` is not set, the default value `http://localhost:8000` will be used.

```sh
export PAPERLESS_API_TOKEN=your_api_token
export PAPERLESS_URL=http://localhost:8000
```

To upload a document, use the following command:

```sh
paperless-cli path/to/your/document.pdf
```

## Configuration

- `PAPERLESS_API_TOKEN`: Your Paperless-ngx API token (required)
- `PAPERLESS_URL`: The URL of your Paperless-ngx instance (optional, default: `http://localhost:8000`)

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on [GitHub](https://github.com/wormi4ok/paperless-cli).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Paperless-ngx](https://github.com/paperless-ngx/paperless-ngx) for providing the API and building an incredibilty useful tool.

## Disclaimer

This project is not affiliated with or endorsed by the Paperless project. Use at your own risk.
