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

### Install as Global Command

For easy access from anywhere on your system:

```bash
# Build the release binary
cargo build --release

# Copy to your PATH (macOS/Linux)
sudo cp target/release/docker-tui /usr/local/bin/dtui

# Now you can run it from any directory
dtui
```

Add an alias to your shell config for convenience (`~/.zshrc` or `~/.bashrc`):

```bash
alias dtui='docker-tui'
```

### Running

```bash
cargo run --release
```

Or if installed globally:

```bash
dtui
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

## Development Workflows

### Daily Development Routine

Start only the services you need for efficient resource usage:

```bash
# Start your project
cd ~/projects/my-app
docker-compose up -d postgres redis

# Launch dtui to monitor
dtui

# During development:
# - Press 's' to start additional services as needed
# - Press 'x' to stop services you're not using
# - Press 'r' to quickly restart a service after configuration changes
```

### Multi-Project Development

When working across multiple projects with different containers:

```bash
# Working on frontend and backend simultaneously
cd ~/projects/frontend-app
docker-compose up -d

cd ~/projects/backend-api
docker-compose up -d

# Launch dtui to see ALL containers from both projects
dtui

# Quickly identify which services belong to which project
# Stop containers from projects you're not actively working on
```

### Hot Reload and Quick Restarts

Perfect for development with file watching:

```bash
# Keep dtui open in a split terminal
# Terminal 1: Your code editor
# Terminal 2: dtui

# When hot reload fails or container needs restart:
# Just press 'r' instead of typing docker restart commands
# Much faster than Docker Desktop GUI
```

### Pre-Deployment Health Checks

Verify everything is working before pushing to production:

```bash
# Start production-like environment
docker-compose -f docker-compose.prod.yml up -d

# Launch dtui
dtui

# Visual health check:
# - All containers should have green circles (healthy)
# - No containers in 'exited' state
# - All expected services running

# If issues found, use 'r' to restart or 'x' to stop problem containers
```

### Debugging Container Issues

When something goes wrong:

```bash
dtui

# Quickly identify problems:
# 1. Red circles = unhealthy containers
# 2. 'exited' status = crashed containers
# 3. Missing expected containers

# Actions:
# - Press 'r' to restart unhealthy services
# - Press 'd' to remove and recreate problematic containers
# - Press 'R' to manually refresh if auto-refresh is slow
```

### Resource Management for Laptops

Optimize battery life and performance:

```bash
dtui

# Stop resource-heavy containers when not needed:
# - Database containers during frontend-only work
# - Elasticsearch or other heavy services
# - Old containers from previous projects

# Free up RAM and CPU without remembering container names
# Navigate with arrows, press 'x' to stop
```

### Remote Server Management

Monitor containers on staging or production servers:

```bash
# SSH into your server
ssh your-staging-server

# Launch dtui
dtui

# Monitor deployed containers
# Restart services experiencing issues
# Works great over slow connections (minimal bandwidth)
```

### Integration with Project Scripts

**Add to package.json:**

```json
{
  "scripts": {
    "docker:monitor": "docker-tui",
    "dev": "docker-compose up -d && docker-tui"
  }
}
```

**Add to Makefile:**

```makefile
.PHONY: monitor
monitor:
	@docker-tui

.PHONY: dev
dev:
	docker-compose up -d
	@docker-tui

.PHONY: clean
clean:
	docker-compose down
```

Run with:
```bash
npm run dev
# or
make dev
```

**Create project-specific script (`dev.sh`):**

```bash
#!/bin/bash
echo "Starting development environment..."
docker-compose up -d
echo "Launching container monitor..."
dtui
```

Make it executable and run:
```bash
chmod +x dev.sh
./dev.sh
```

### Terminal Multiplexer Workflow

Use with tmux or screen for powerful layouts:

```bash
# Create a development session
tmux new-session -d -s dev 'docker-compose up'
tmux split-window -h 'dtui'
tmux split-window -v 'npm run dev'
tmux attach -t dev

# Now you have:
# - Left pane: Docker logs
# - Top right: Container monitor (dtui)
# - Bottom right: Your application
```

### CI/CD Integration

Use for automated health checks:

```bash
#!/bin/bash
# deploy-check.sh

# Deploy containers
docker-compose up -d

# Wait for startup
sleep 10

# Quick health verification
# (You could extend dtui to support a --check flag that exits with status code)
docker ps --format "table {{.Names}}\t{{.Status}}" | grep -i "unhealthy" && exit 1

echo "All containers healthy!"
```

## Real-World Example: Full-Stack Application

Typical workflow for a full-stack app with multiple services:

```bash
# Morning: Start essential services only
cd ~/projects/my-saas-app
docker-compose up -d postgres redis
dtui  # Verify they're running (green status)

# Frontend work: Backend not needed
# Keep backend container stopped to save 2GB RAM

# Need to test API integration
# In dtui: Navigate to backend container, press 's' to start
# Takes 2 seconds vs 30 seconds in Docker Desktop

# Backend acting weird
# In dtui: Press 'r' to restart
# Faster than: docker ps -> copy ID -> docker restart <ID>

# Switching to backend development
# In dtui: Stop frontend container, keep backend running

# End of day: Clean up
# In dtui: Press 'x' on each running container
# Or use: docker-compose down
```

### Advantages Over Docker Desktop

**Speed:**
- Launch: <1 second vs 5-10 seconds for Docker Desktop
- Navigation: Instant keyboard control vs mouse clicking
- Actions: Single keypress vs multiple clicks

**Efficiency:**
- Minimal memory footprint
- Works over SSH connections
- No GUI overhead
- Keyboard-driven workflow

**Developer Experience:**
- Stay in terminal, no context switching
- Integrates with terminal multiplexers
- Scriptable and automatable
- Works on servers without GUI

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

## Tips and Tricks

### Quick Access Setup

Add this function to your shell config for instant access:

```bash
# ~/.zshrc or ~/.bashrc
function dmon() {
  if command -v dtui &> /dev/null; then
    dtui
  else
    echo "dtui not installed. Run: cargo install --path /path/to/docker-tui"
  fi
}
```

### Project-Specific Container Groups

Create aliases for different project setups:

```bash
alias frontend-dev='cd ~/projects/frontend && docker-compose up -d && dtui'
alias backend-dev='cd ~/projects/backend && docker-compose up -d && dtui'
alias fullstack-dev='cd ~/projects/app && docker-compose up -d && dtui'
```

### Health Check Best Practices

For optimal health status monitoring, add health checks to your Dockerfile:

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=40s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

Or in docker-compose.yml:

```yaml
services:
  api:
    image: my-api
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 40s
```

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
