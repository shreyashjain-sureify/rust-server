# Rust HTTP Server

This project is a simple HTTP server written in Rust.

## Features

- Handles HTTP requests using a custom server implementation.
- Serves static files from a specified directory.
- Supports multiple HTTP methods.

## Usage

To use this server, you can clone the repository and run it locally.

### Prerequisites

- Rust toolchain (https://www.rust-lang.org/tools/install)
- Docker (https://www.docker.com/get-started)

### Building the Server

To build the server locally, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/your_username/rust-server.git
    cd rust-server
    ```

2. Build the server using Cargo:

    ```bash
    cargo build --release
    ```

### Running the Server

To run the server locally, execute the following command:

```bash
cargo run
```

By default, the server will listen on 127.0.0.1:8085.

### Running with Docker

You can also run the server using Docker. A Dockerfile is provided in the repository for this purpose.

### Build the Docker image:

    ```bash
    docker build -t rust-server .
    ```

### Run the Docker container:

    ```bash
    docker run -p 8085:8085 rust-server
    ```

The server will now be accessible at http://localhost:8085.

