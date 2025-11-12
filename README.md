# Docker TUI

A beautiful, fast terminal user interface for managing Docker containers built with Rust.

## Features

- Real-time container monitoring with auto-refresh
- Color-coded container status (running, stopped, paused)
- Health status indicators for containers with health checks
- Keyboard-driven navigation for efficient container management
- Confirmation dialogs for destructive operations
- Start, stop, restart, and delete containers
- Minimal resource usage with async operations

## Prerequisites

- Rust 1.70 or higher
- Docker Desktop or Docker Engine running locally
- macOS, Linux, or Windows with WSL2

## Installation

### From Source
```bash
git clone https://github.com/yourusername/docker-tui.git
cd docker-tui
cargo build --release
```

The compiled binary will be available at `target/release/docker-tui`.

### Running
```bash
cargo run --release
```

Or run the binary directly:
```bash
./target/release/docker-tui
```

## Usage

### Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `k` | Navigate up |
| `↓` / `j` | Navigate down |
| `s` | Start selected container |
| `x` | Stop selected container |
| `r` | Restart selected container |
| `d` | Delete selected container (with confirmation) |
| `R` | Manual refresh container list |
| `q` | Quit application |

### Container Information Display

The interface shows the following information for each container:

- Container name
- Health status indicator (green for healthy, red for unhealthy, yellow for starting)
- Container ID (truncated to 12 characters)
- Current status (running, exited, paused, etc.)
- Image name

### Health Status Indicators

- Green circle: Container is healthy
- Red circle: Container is unhealthy
- Yellow half-circle: Health check is starting
- No indicator: Container has no health check configured

## Architecture

### Project Structure
```
docker-tui/
├── src/
│   ├── main.rs           # Application entry point and event loop
│   ├── docker/
│   │   ├── mod.rs        # Docker module exports
│   │   └── client.rs     # Docker API client wrapper
│   └── ui/
│       ├── mod.rs        # UI module exports
│       ├── app.rs        # Application state management
│       └── render.rs     # UI rendering logic
├── Cargo.toml            # Dependencies and project metadata
└── README.md
```

### Key Technologies

- **ratatui**: Modern TUI framework for building terminal interfaces
- **crossterm**: Cross-platform terminal manipulation
- **tokio**: Async runtime for handling Docker API calls
- **bollard**: Official Docker Engine API client
- **anyhow**: Simplified error handling

## Development

### Building for Development
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Performance

The application polls the Docker API every 500ms for container updates, providing near real-time status information while maintaining minimal CPU and memory usage. All Docker operations are performed asynchronously to keep the UI responsive.

## Troubleshooting

### Docker Connection Issues

If the application fails to start with a Docker connection error:

1. Ensure Docker Desktop or Docker Engine is running
2. Verify Docker socket is accessible: `docker ps`
3. Check Docker socket permissions (Linux/macOS)

### Slow Status Updates

Container stop operations may take 3-5 seconds due to Docker's graceful shutdown process. This is normal behavior as Docker sends SIGTERM and waits for the process to exit cleanly.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Development Guidelines

- Follow Rust standard formatting (use `cargo fmt`)
- Ensure all tests pass before submitting PRs
- Add tests for new features
- Update documentation as needed

## Learning Resources

This project is an excellent introduction to:

- Rust programming fundamentals
- Async/await patterns in Rust
- Terminal UI development
- Docker API integration
- Systems programming

### Key Rust Concepts Demonstrated

- Ownership and borrowing
- Error handling with Result types
- Pattern matching
- Async/await with tokio
- Modular code organization
- Trait implementations

## License

MIT License - feel free to use this project for learning or as a base for your own tools.

## Acknowledgments

Built as a learning project to explore Rust, async programming, and terminal UI development. Special thanks to the ratatui and bollard communities for excellent documentation and examples.
