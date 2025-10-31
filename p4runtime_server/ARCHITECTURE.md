# P4Runtime Server Architecture Options

This document explores several architectural approaches for integrating the P4Runtime server with x4c's Pipeline trait. Each option is evaluated based on complexity, flexibility, performance, and maintainability.

## Key Constraints & Requirements

- **x4c Compilation**: P4 programs are compiled at build time via `use_p4!` macro or `x4c` CLI
- **Pipeline Trait**: x4c-generated pipelines implement the `p4rs::Pipeline` trait with qualified table names (e.g., `"ingress.router.ipv6_routes"`)
- **P4Runtime**: Uses numeric table IDs from P4Info, requires ForwardingPipelineConfig (P4Info + device config)
- **Table Mapping**: Need to map P4Runtime table IDs → x4c qualified table names
- **Packet I/O**: Pipeline processes packets via `process_packet()` returning `Vec<(packet_out, u16)>`
- **Dynamic Loading**: x4c supports `dlopen` via C FFI (`_main_pipeline_create` function)

---

## Option 1: Static Compile-Time Integration (Single Pipeline)

### Architecture

Pre-compile one or more specific P4 programs directly into the P4Runtime server at build time using the `use_p4!` macro.

```rust
// In p4runtime_server/src/main.rs
p4_macro::use_p4!("p4_programs/my_router.p4", pipeline_name = "router");

struct P4RuntimeService {
    pipeline: Mutex<router_pipeline>,
    p4info: Option<P4Info>,  // From SetForwardingPipelineConfig
    id_to_name_map: HashMap<u32, String>,  // Table ID → qualified name
}
```

### Pros
- ✅ **Simplest to implement** - Direct compilation, no runtime complexity
- ✅ **Type-safe** - Compile-time checking of P4 program validity
- ✅ **Best performance** - No compilation or loading overhead
- ✅ **Easiest debugging** - Full IDE support and compile-time errors
- ✅ **No external dependencies** - Everything statically linked

### Cons
- ❌ **No runtime flexibility** - P4 program fixed at compile time
- ❌ **Limited to predefined programs** - Can't load arbitrary P4 programs
- ❌ **Requires server rebuild** - To change or add P4 programs
- ❌ **Single pipeline constraint** - One program per server instance (or hardcode multiple)

### Best For
- Fixed-function servers with known P4 programs
- Development and testing environments
- Production deployments with stable P4 programs

---

## Option 2: Static Compile-Time Integration (Multi-Pipeline)

### Architecture

Pre-compile multiple P4 programs into the server, select which to use based on P4Runtime config.

```rust
// In p4runtime_server/src/main.rs
p4_macro::use_p4!("p4_programs/router.p4", pipeline_name = "router");
p4_macro::use_p4!("p4_programs/firewall.p4", pipeline_name = "firewall");
p4_macro::use_p4!("p4_programs/nat.p4", pipeline_name = "nat");

enum PipelineType {
    Router(router_pipeline),
    Firewall(firewall_pipeline),
    Nat(nat_pipeline),
}

struct P4RuntimeService {
    pipeline: Mutex<Option<PipelineType>>,
    p4info: Option<P4Info>,
    id_to_name_map: HashMap<u32, String>,
}

// Select pipeline based on P4Info program name or other identifier
```

### Pros
- ✅ **Multiple programs** - Support different pipelines without restart
- ✅ **Type-safe** - Each program validated at compile time
- ✅ **Good performance** - No runtime compilation
- ✅ **Known programs** - Can optimize for specific programs

### Cons
- ❌ **Still limited** - Only programs compiled into the server
- ❌ **Large binary** - Includes all pipeline code
- ❌ **Complexity** - Need mapping logic for program selection
- ❌ **Requires rebuild** - To add new programs

### Best For
- Servers with a known set of P4 programs
- Use cases where programs are selected at startup but rarely changed

---

## Option 3: Dynamic Loading (Pre-compiled Shared Libraries)

### Architecture

Compile P4 programs as separate shared libraries (.so/.dylib), dynamically load them via `dlopen` when `SetForwardingPipelineConfig` is called.

```rust
use libloading::Library;

struct LoadedPipeline {
    library: Library,
    pipeline: Box<dyn Pipeline>,
    create_fn: unsafe extern "C" fn(u16) -> *mut dyn Pipeline,
}

struct P4RuntimeService {
    pipeline: Mutex<Option<LoadedPipeline>>,
    p4info: Option<P4Info>,
    id_to_name_map: HashMap<u32, String>,
    pipeline_cache: HashMap<String, LoadedPipeline>,  // Optional: cache loaded pipelines
}

// In SetForwardingPipelineConfig:
// 1. Extract P4Info to determine program name
// 2. Resolve path to .so/.dylib file
// 3. Load library via libloading
// 4. Get _main_pipeline_create symbol
// 5. Create pipeline instance
// 6. Build ID-to-name mapping from P4Info
```

### Pros
- ✅ **Runtime flexibility** - Load different P4 programs without rebuilding server
- ✅ **Modular** - Each P4 program is a separate shared library
- ✅ **Reusable libraries** - Same library can be used by multiple server instances
- ✅ **Moderate complexity** - More complex than static but manageable
- ✅ **Hot-swappable** - Can replace pipeline at runtime (with proper locking)

### Cons
- ❌ **Two-stage build** - Need to compile P4 programs separately first
- ❌ **Library management** - Need infrastructure to compile/build shared libraries
- ❌ **Runtime errors** - Loading failures happen at runtime, not compile-time
- ❌ **Platform-specific** - Different library formats (.so vs .dylib)
- ❌ **Symbol management** - Need to handle symbol versioning and compatibility

### Best For
- Flexible deployment scenarios
- Development environments with frequently changing P4 programs
- Production where programs are updated without server downtime

---

## Option 3a: Dynamic Loading via `p4_device_config` (Embedded Library)

### Architecture

Send the compiled shared library binary directly in the `p4_device_config` field of `SetForwardingPipelineConfig`. Write it to a temporary file and load it dynamically.

```rust
use libloading::Library;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

struct LoadedPipeline {
    library: Library,
    pipeline: Box<dyn Pipeline>,
    temp_file: Option<NamedTempFile>,  // Keep reference to prevent cleanup
}

struct P4RuntimeService {
    pipeline: Mutex<Option<LoadedPipeline>>,
    p4info: Option<P4Info>,
    id_to_name_map: HashMap<u32, String>,
}

// In SetForwardingPipelineConfig:
async fn set_forwarding_pipeline_config(
    &self,
    request: Request<SetForwardingPipelineConfigRequest>,
) -> Result<Response<SetForwardingPipelineConfigResponse>, Status> {
    let req = request.into_inner();
    let config = req.config.ok_or_else(|| Status::invalid_argument("missing config"))?;
    
    // Extract P4Info
    let p4info = config.p4info.ok_or_else(|| Status::invalid_argument("missing p4info"))?;
    
    // Extract shared library from p4_device_config
    let lib_bytes = config.p4_device_config;
    
    // Write to temporary file and load
    let temp_file = NamedTempFile::new()
        .map_err(|e| Status::internal(format!("failed to create temp file: {}", e)))?;
    
    // Write library bytes to temp file
    fs::write(temp_file.path(), &lib_bytes)
        .map_err(|e| Status::internal(format!("failed to write library: {}", e)))?;
    
    // Load library
    let library = Library::new(temp_file.path())
        .map_err(|e| Status::invalid_argument(format!("failed to load library: {}", e)))?;
    
    // Get pipeline creation function
    let create_fn: libloading::Symbol<unsafe extern "C" fn(u16) -> *mut dyn Pipeline> = 
        unsafe { library.get(b"_main_pipeline_create") }
            .map_err(|e| Status::invalid_argument(format!("missing _main_pipeline_create: {}", e)))?;
    
    // Create pipeline instance
    let pipeline = unsafe { Box::from_raw(create_fn(0)) };  // radix = 0
    
    // Build ID to name mappings from P4Info
    let id_to_name_map = build_table_id_mapping(&p4info)?;
    
    // Store loaded pipeline
    let mut current = self.pipeline.lock().unwrap();
    *current = Some(LoadedPipeline {
        library,
        pipeline,
        temp_file: Some(temp_file),
    });
    
    Ok(Response::new(SetForwardingPipelineConfigResponse {}))
}
```

### Pros
- ✅ **Maximum flexibility** - Controller sends fully compiled pipeline as binary
- ✅ **No file system dependencies** - No need for shared storage or file paths
- ✅ **Self-contained** - Everything needed in the P4Runtime request
- ✅ **Easy deployment** - Controller can compile and deploy in one step
- ✅ **Works across environments** - No shared filesystem requirements
- ✅ **Atomic updates** - Entire pipeline in one request

### Cons
- ❌ **Large payload** - Shared libraries can be several MB
- ❌ **Security concerns** - Loading arbitrary binary code requires strong safeguards
- ❌ **ABI compatibility** - Must ensure library was compiled with compatible toolchain
- ❌ **Temporary file management** - Need to handle cleanup and file lifetimes
- ❌ **Platform detection** - Server must know expected library format (.so vs .dylib)

### Security Considerations

This approach loads arbitrary binary code. For production use, implement input validation (library format checks), size limits, ABI versioning, and proper error isolation. For development, basic validation and error handling should suffice.

### Best For
- **Development environments** where maximum flexibility is desired
- **Self-contained deployments** without shared filesystem
- **Rapid iteration** where P4 programs change frequently
- **Testing** where you want to test different pipelines easily

---

## Option 4: Hybrid: Static Fallback + Dynamic Loading

### Architecture

Combine static compilation for common programs with dynamic loading as fallback.

```rust
enum PipelineType {
    StaticRouter(Mutex<router_pipeline>),
    StaticFirewall(Mutex<firewall_pipeline>),
    Dynamic(LoadedPipeline),
}

struct P4RuntimeService {
    pipeline: Mutex<Option<PipelineType>>,
    // ... rest of fields
}

// Try static pipelines first, fall back to dynamic loading
```

### Pros
- ✅ **Best of both worlds** - Performance for common programs, flexibility for others
- ✅ **Gradual adoption** - Start with static, add dynamic later
- ✅ **Optimization path** - Can optimize common statically-compiled programs

### Cons
- ❌ **Most complex** - Two code paths to maintain
- ❌ **Larger codebase** - Includes both static and dynamic logic
- ❌ **Decision complexity** - Need logic to choose static vs dynamic

### Best For
- Production systems with a few common programs plus occasional custom ones
- Systems migrating from static to dynamic approach

---

## Option 5: Adapter/Wrapper Layer

### Architecture

Create an abstraction layer that translates between P4Runtime concepts and x4c Pipeline trait, independent of how pipelines are loaded.

```rust
struct PipelineAdapter {
    pipeline: Box<dyn Pipeline>,
    p4info: P4Info,
    id_to_name: HashMap<u32, String>,  // Table ID → qualified name
    name_to_id: HashMap<String, u32>,  // Qualified name → table ID
    action_id_to_name: HashMap<u32, String>,
}

impl PipelineAdapter {
    fn from_pipeline(pipeline: Box<dyn Pipeline>, p4info: P4Info) -> Self {
        // Build mappings from P4Info
        // ...
    }
    
    fn map_table_id(&self, id: u32) -> Option<&str> {
        self.id_to_name.get(&id).map(|s| s.as_str())
    }
    
    fn write_table_entry(&mut self, req: &TableEntry) -> Result<()> {
        let table_name = self.map_table_id(req.table_id)?;
        let action_name = self.action_id_to_name.get(&req.action.action_id)?;
        self.pipeline.add_table_entry(
            table_name,
            action_name,
            &req.match_[..].to_bytes(),  // Convert match fields
            &req.action.params.to_bytes(),
            req.priority,
        );
        Ok(())
    }
}
```

### Pros
- ✅ **Decouples loading** - Adapter works with any loading mechanism
- ✅ **Clean separation** - P4Runtime logic separate from pipeline implementation
- ✅ **Testable** - Can unit test adapter logic independently
- ✅ **Reusable** - Adapter can be used with any Option above

### Cons
- ❌ **Additional layer** - Extra abstraction and translation overhead
- ❌ **Mapping complexity** - Need robust ID ↔ name translation
- ❌ **Data conversion** - Convert P4Runtime formats to x4c formats

### Best For
- **Recommended for all options** - Should be used regardless of loading mechanism
- Clean architecture with separation of concerns
- Makes testing and maintenance easier

---

## Comparison Matrix

| Aspect | Static Single | Static Multi | Dynamic Loading | Hybrid | Adapter Layer |
|--------|--------------|-------------|----------------|--------|---------------|
| **Complexity** | ⭐ Very Low | ⭐⭐ Low | ⭐⭐⭐ Medium | ⭐⭐⭐⭐ High | ⭐⭐ Low-Med |
| **Flexibility** | ⭐ Very Low | ⭐⭐ Low | ⭐⭐⭐⭐ High | ⭐⭐⭐⭐ High | N/A (works with all) |
| **Performance** | ⭐⭐⭐⭐⭐ Best | ⭐⭐⭐⭐⭐ Best | ⭐⭐⭐⭐ Good | ⭐⭐⭐⭐⭐ Best (for static) | ⭐⭐⭐⭐ Good |
| **Build Time** | ⭐⭐⭐⭐ Fast | ⭐⭐⭐⭐ Fast | ⭐⭐⭐ Slow (two-stage) | ⭐⭐⭐ Medium | ⭐⭐⭐⭐ Fast |
| **Runtime Safety** | ⭐⭐⭐⭐⭐ Best | ⭐⭐⭐⭐⭐ Best | ⭐⭐⭐ Medium | ⭐⭐⭐⭐ Good | ⭐⭐⭐⭐ Good |
| **Development Speed** | ⭐⭐⭐⭐⭐ Fastest | ⭐⭐⭐⭐ Fast | ⭐⭐⭐ Medium | ⭐⭐ Slow | ⭐⭐⭐⭐ Fast |
| **Maintenance** | ⭐⭐⭐⭐⭐ Easiest | ⭐⭐⭐⭐ Easy | ⭐⭐⭐ Medium | ⭐⭐ Hard | ⭐⭐⭐ Medium |

---

## Recommended Approach

### Phase 1: Static Single + Adapter Layer (MVP)

Start with **Option 1** (Static Single) combined with **Option 5** (Adapter Layer):

1. Pre-compile one P4 program using `use_p4!` macro
2. Implement `PipelineAdapter` to translate P4Runtime ↔ x4c
3. This gives you a working server quickly with clean architecture

**Benefits**: Fastest to implement, type-safe, easy to debug, establishes good patterns

### Phase 2: Add Dynamic Loading (Production)

Once MVP is working, add **Option 3** (Dynamic Loading):

1. Keep static pipeline as default/fallback
2. Add infrastructure to compile P4 programs to shared libraries
3. Add dynamic loading path in `SetForwardingPipelineConfig`
4. Reuse existing `PipelineAdapter` layer

**Benefits**: Flexibility without sacrificing architecture quality

---

## Implementation Considerations

### Table ID to Name Mapping

P4Runtime uses numeric table IDs from P4Info, while x4c uses qualified names. The adapter layer must:

1. Parse P4Info to extract table metadata
2. Map table IDs to qualified names using P4Info annotations or naming conventions
3. Handle reverse lookup (name → ID) for responses

### Action ID Mapping

Similar challenge with actions:
- P4Runtime: Numeric action IDs
- x4c: Action names (e.g., `"forward_out_port"`)

### Key/Parameter Format Conversion

- P4Runtime match fields may need conversion to x4c's byte format
- Action parameters need translation between formats

### Packet I/O Integration

For `StreamChannel`:
- Receive packets from controller
- Convert to `packet_in` format
- Call `pipeline.process_packet()`
- Convert outputs back to `StreamMessageResponse`
- Handle bidirectional streaming

---

## Next Steps

1. **Implement Option 1 + Option 5** (Static + Adapter) as MVP
2. **Add P4Info parsing** to build ID mappings
3. **Implement Capabilities** returning version info
4. **Implement SetForwardingPipelineConfig** to store P4Info
5. **Implement Write/Read** for table operations via adapter
6. **Add integration tests** with real P4 programs
7. **Evaluate need for dynamic loading** based on use cases

