![Dynamic logo](/assets/svg/icon-secondary.svg)

# DynamicSDK Admin Operations CLI

A Rust-based command-line interface tool for interacting with the DynamicSDK (Dynamic.xyz) Admin API. This tool provides an interactive way to perform admin operations such as managing organizations, exports, users, and more.

## Features

- Interactive command selection using a dialog interface
- Command-line argument support for automation
- Modular design using the Command pattern for easy extension
- Supports Organizations, Exports, and Users API operations
- Configuration management with secure token storage
- Documentation: https://docs.dynamic.xyz/api-reference/overview

## Installation

### Prerequisites

- Rust and Cargo (install via [rustup](https://rustup.rs/))

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/brohamgoham/dynamic-admin-rs.git
cd dynamic-admin-ops
```

2. Build the project:
```bash
cargo build --release
```

3. The binary will be available at `target/release/dynamic-admin-ops`

## Configuration

On first run, the CLI will prompt you for configuration information:

- API Token: Your DynamicSDK API token (starts with `dyn_`)
- Base URL: The base URL for the API (default is `https://app.dynamicauth.com`)
- Default Environment ID: Your DynamicSDK environment ID

Alternatively, you can manually create a configuration file at:
- Linux/macOS: `~/.config/dynamic-admin-ops/config.toml`
- Windows: `%APPDATA%\dynamic-admin-ops\config.toml`

See the `config.toml.sample` file for the required format.

## Usage

### Interactive Mode

Simply run the tool without arguments to enter interactive mode:

```bash
./dynamic-admin
```

You'll be presented with a menu to select categories and operations.

### Command-line Mode

You can also specify commands directly as arguments:

```bash
./dynamic-admin organizations list
./dynamic-admin exports get
```

## Available Commands

### Organizations
- `list`: List all organizations
- `get`: Get a specific organization by ID

### Exports
- `list`: List all exports
- `get`: Get a specific export by ID
- `create`: Create a new export

### Users
- `list`: List all users
- `get`: Get a specific user by ID

## Extending the Tool

The tool is designed to be easily extensible using the Command pattern:

1. Add new API methods to `src/api.rs`
2. Create a new command implementation in the appropriate module
3. Register the command in the `create_command_registry` function in `src/commands.rs`

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.