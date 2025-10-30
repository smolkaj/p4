# P4Runtime Server

A work-in-progress gRPC server implementing the [P4Runtime specification](https://p4.org/specifications/).

## Overview

This server provides a P4Runtime service that allows controllers to configure and manage P4 data plane programs. The server currently implements a skeleton with all required RPC methods, each returning "unimplemented" status until business logic is added.

> **New to gRPC in Rust?** Check out this [video introduction to gRPC in Rust](https://www.youtube.com/watch?v=kerKXChDmsE) for a quick way to familiarize yourself with the concepts and patterns used in this project.

## Developer Workflow

### Building

Build the server:

```bash
cargo build
```

### Running

Start the server:

```bash
cargo run
```

The server will start on `[::1]:9559` (IPv6 localhost, port 9559).

## Manual Testing & Tinkering

The server supports [gRPC reflection](https://grpc.io/docs/guides/reflection/), which allows clients to discover services and methods dynamically without needing the `.proto` files. This makes explorative testing and debugging fun and easy.

### Using grpcui

`grpcui` provides a web-based UI for interacting with gRPC services.

1. **Install grpcui**:
   ```bash
   # macOS
   brew install grpcui

   # Or build from source
   go install github.com/fullstorydev/grpcui/cmd/grpcui@latest
   ```

2. **Start the server** (in one terminal):
   ```bash
   cargo run
   ```

3. **Launch grpcui** (in another terminal):
   ```bash
   grpcui -plaintext localhost:9559
   ```

4. **Use the web interface**:  Try sending a request and see what happens.

### Using grpcurl

`grpcurl` is a command-line tool for interacting with gRPC services.

1. **Install grpcurl**:
   ```bash
   # macOS
   brew install grpcurl
   
   # Or build from source
   go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
   ```

2. **Call a method**:
   ```bash
   grpcurl -plaintext localhost:9559 p4.v1.P4Runtime.Write -d "<my request>"
   ```
   
   This will call the `Write` method with `<my request>` specified in text format.


3. **List available services**:
   ```bash
   grpcurl -plaintext localhost:9559 list
   ```
   
   This should show:
   - `grpc.reflection.v1.ServerReflection`
   - `p4.v1.P4Runtime`

4. **List methods in a service**:
   ```bash
   grpcurl -plaintext localhost:9559 list p4.v1.P4Runtime
   ```
   
   This will list all P4Runtime methods:
   - `Capabilities`
   - `GetForwardingPipelineConfig`
   - `Read`
   - `SetForwardingPipelineConfig`
   - `StreamChannel`
   - `Write`
