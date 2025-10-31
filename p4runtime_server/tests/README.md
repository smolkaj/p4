# P4Runtime Server End-to-End Tests

These tests use `grpcurl` and `trycmd` to test the P4Runtime server end-to-end, by issuing requests and observing the resulting responses.

## How It Works
1. **Test Cases** (`*_test.md` files): Define commands and expected output in markdown
2. **Test Setup** (`e2e_tests.rs`): For each registered markdown file, starts a fresh server instance and invokes `trycmd`.
3. **trycmd** (`e2e_tests.rs`): Executes the commands sequentially and verifies output matches expectations

## Writing Tests

Tests are markdown files with `console` code blocks containing:
- Command with `$` prompt
- Expected output below

See `p4runtime_test.md` for examples.

You can generate the expected output automatically by running

```bash
TRYCMD=update cargo test --test e2e_tests
```

## Running Tests

```bash
# Run the tests:
cargo test --test e2e_tests

# Update the exepcted outputs automatically:
TRYCMD=update cargo test --test e2e_tests  
```

