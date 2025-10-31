# Known Problems

This document tracks known problems and limitations in the p4runtime_server codebase.

## Silent Duplicate Entry Overwrites on Insert

**Status**: Open problem

**Problem**: When a P4Runtime Write operation has `UpdateType::Insert`, the server cannot detect if the table entry already exists. The pipeline's `add_table_entry()` method silently overwrites existing entries rather than returning an error.

**Impact**:
- P4Runtime `INSERT` semantics are violated: Insert should fail if the entry exists, but it silently succeeds by overwriting
- Clients cannot distinguish between a successful insert of a new entry vs. an accidental overwrite of an existing entry
- This violates P4Runtime specification behavior for INSERT operations

**Root Cause**: The `p4rs::Pipeline::add_table_entry()` method returns `()` and doesn't distinguish between insert and modify operations. The table implementation (likely a `HashMap` or similar) treats insert as "upsert" behavior.

**Potential Solutions**:
1. Add a method to `p4rs::Pipeline` trait: `table_entry_exists(table_id: &str, keyset_data: &[u8]) -> bool` to check before inserting
2. Modify `add_table_entry` to return `Result<(), PipelineError>` and distinguish insert vs modify
3. Change the table implementation to track insert vs modify semantics (would require architectural changes)

**Related Code**:
- `p4runtime_server/src/main.rs:304-312` - Insert operations call `add_table_entry` without existence check
- `lang/p4rs/src/lib.rs:170-177` - Pipeline trait definition
- `lang/p4rs/src/table.rs` - Table implementation (likely uses HashMap semantics)

---

## Silent Internal Table Operation Errors

**Status**: Open problem

**Problem**: The `p4rs::Pipeline` trait methods `add_table_entry()` and `remove_table_entry()` return `()` (void), so any internal errors that occur during table operations cannot be detected or reported to the P4Runtime client.

**Potential Failure Modes**:
- Memory allocation failures during entry insertion
- Table corruption or internal state errors
- Concurrent modification conflicts (if tables become thread-safe in the future)
- Resource exhaustion (table size limits, memory pressure)

**Impact**:
- P4Runtime Write operations may appear to succeed when internal failures occurred
- No way to diagnose issues without access to logs or debug output
- Silent data corruption if table state becomes inconsistent

**Root Cause**: The Pipeline trait design prioritizes simplicity over error reporting. Internal errors are either panicked (if critical) or silently ignored (if non-critical).

**Potential Solutions**:
- Modify `p4rs::Pipeline` trait to return `Result<(), PipelineError>` for all table operations
- Update codegen to propagate errors from table implementations
- Add instrumentation/logging to detect silent failures (temporary workaround)

**Related Code**:
- `lang/p4rs/src/lib.rs:170-180` - Pipeline trait definition
- `codegen/rust/src/pipeline.rs` - Generated pipeline code

---

## Background: Why Most Validation Works

**Status**: Known design choice, generally sufficient

The P4Runtime layer performs comprehensive validation before calling pipeline methods:

1. **Comprehensive validation at P4Runtime layer**: 
   - Table IDs are validated via `translator.map_table_id()` which ensures the table exists (the translator is built from the actual pipeline via `pipeline.get_table_ids()`)
   - Action IDs are validated via `translator.map_action_id()` 
   - Update structure, match fields, and action types are all validated before calling pipeline methods

2. **Translator guarantees correctness**: The translator is constructed from the actual pipeline instance, ensuring that any table/action ID that successfully maps through the translator is guaranteed to exist in the pipeline. The generated code's `println!` for unknown tables will never trigger for validated inputs.

3. **Format validation happens implicitly**: If `keyset_data` or `parameter_data` are malformed, the table implementation will handle it appropriately (either by rejecting during lookup or by design).

**Conclusion**: Most validation concerns are addressed at the P4Runtime layer. However, the two problems above represent gaps where pipeline-level behavior cannot be validated or reported.
