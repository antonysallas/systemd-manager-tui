# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust TUI (Terminal User Interface) application for managing systemd services. It provides an interactive interface to view, control, and monitor systemd units using the D-Bus API via zbus. The application supports both system and session units with Vim-like navigation.

## Build and Development Commands

### Standard Development
```bash
# Build the project
cargo build --release

# Run the application
cargo run

# Run with release optimizations
cargo run --release

# Install via cargo
cargo install --locked systemd-manager-tui
```

### Cross-Platform Release Build
The project includes a comprehensive build script for generating distribution packages:
```bash
# Build for multiple architectures and generate .deb and .rpm packages
./build.sh
```

This script builds for:
- Native target (x86_64)
- `x86_64-unknown-linux-musl` (static binary)
- `aarch64-unknown-linux-musl` (ARM64 static binary)

And generates:
- `.deb` packages using `cargo-deb`
- `.rpm` packages using `cargo-generate-rpm`

Build artifacts are placed in:
- `target/release/` - Native build
- `target/{target}/debian/` - Debian packages
- `target/{target}/generate-rpm/` - RPM packages

## Architecture

The codebase follows a **clean architecture** pattern with clear separation of concerns:

### Domain Layer (`src/domain/`)
Core business logic, completely independent of external dependencies:
- **`service.rs`**: Service entity with name, description, and state
- **`service_state.rs`**: Service state (load, active, sub, file states)
- **`service_repository.rs`**: Repository trait defining all systemd operations (list, start, stop, restart, enable, disable, mask, unmask, get logs, etc.)

### Infrastructure Layer (`src/infrastructure/`)
External integrations and adapters:
- **`systemd_service_adapter.rs`**: Implements `ServiceRepository` trait using zbus for D-Bus communication with systemd. Supports both `ConnectionType::System` and `ConnectionType::Session` for managing system and user units.

### Use Cases Layer (`src/usecases/`)
Application-specific business rules:
- **`services_manager.rs`**: Orchestrates domain operations. Handles service lifecycle operations and coordinates between the repository and UI. Key method: `list_services()` merges runtime units and unit files, deduplicates, and filters by type.

### Terminal/Presentation Layer (`src/terminal/`)
- **`app.rs`**: Main application orchestrator with event loop, keyboard handling, tab management (System/Session), and view state management (List/Log/Details). Uses mpsc channels for event communication between UI components.
  - Event system: `AppEvent` enum routes keyboard events and actions between components
  - View states: `Status` enum (List, Log, Details)
  - External editor integration: `edit_unit()` temporarily exits TUI to run `systemctl edit`

- **Components** (`src/terminal/components/`):
  - `list.rs`: Service list table with filtering, sorting, and service actions
  - `log.rs`: Service log viewer (journalctl output)
  - `details.rs`: Service unit file details viewer
  - `filter.rs`: Input component for filtering services with different modes (all, active, inactive, failed)

### Communication Pattern
The application uses an event-driven architecture:
1. Components share `Rc<RefCell<ServicesManager>>` for use case operations
2. Communication via `mpsc::channel<AppEvent>` for UI events
3. Actions trigger state updates that propagate through the component tree
4. Main event loop in `app.rs` dispatches events to appropriate components based on current `Status`

### Key Abstractions
- **Repository Pattern**: `ServiceRepository` trait allows swapping systemd backend without changing business logic
- **Shared Ownership**: `Rc<RefCell<T>>` used for shared mutable state between components
- **Event-Driven**: All UI interactions flow through the event channel system

## Main Dependencies

- **ratatui 0.29**: TUI framework for building the interface
- **zbus 5.5.0**: D-Bus communication for systemd operations
- **crossterm 0.28.1**: Terminal manipulation and event handling
- **color-eyre 0.6.3**: Error handling and reporting
- **chrono 0.4**: Date/time handling for logs

## D-Bus Integration

All systemd operations go through D-Bus:
- Manager proxy: `org.freedesktop.systemd1` at `/org/freedesktop/systemd1`
- Operations call methods like `ListUnits`, `StartUnit`, `StopUnit`, `GetUnitFileState`
- Connection types: System bus for system units, Session bus for user units
- Error handling includes user-friendly messages for common D-Bus errors (authorization required, service unknown, access denied)

## Testing the Application

The application requires a systemd-based Linux system to run since it interacts with systemd via D-Bus. Testing typically involves:
1. Building and running the application
2. Manually testing UI interactions and service operations
3. Verifying both system and session unit management
4. Testing with and without sudo for permission scenarios

Some operations (like starting/stopping system services) may require elevated privileges.
