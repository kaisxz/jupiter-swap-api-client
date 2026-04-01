---
inclusion: fileMatch
fileMatchPattern: "**/*.rs"
---

# Rust Clean Code & Best Practices

## Code Quality and Maintainability

- Write clean, maintainable code
- Follow Clean Code principles
- Use descriptive variable and function names
- Keep functions small and focused (Single Responsibility)
- Avoid code duplication (DRY principle)

## Import Guidelines

**IMPORTANT: Avoid fully qualified namespaces in usage**

❌ Bad:
```rust
let config = solana_client::rpc_config::CommitmentConfig::confirmed();
```

✅ Good:
```rust
use solana_client::rpc_config::CommitmentConfig;

let config = CommitmentConfig::confirmed();
```

### Tracing Crate Exception

- Never use `use tracing::*` imports (e.g. `use tracing::info;`, `use tracing::{info, error};`)
- Always use fully qualified calls: `tracing::info!(...)`, `tracing::error!(...)`, `tracing::debug!(...)`, `tracing::warn!(...)`
- This exception applies only to the `tracing` crate — all other crates follow the normal import rules above

### Import Organization

- Group imports logically: std → external crates → internal modules
- Use `use` statements for all types being used
- Avoid wildcard imports (`use module::*;`) except for preludes
- Import traits explicitly when their methods are used
- Use type aliases for clarity when importing types with same names from different modules

When importing types with the same name from different modules, use type aliases:

❌ Bad (fully qualified paths in code):
```rust
fn convert(event: &parser::types::TradeEvent) -> persistence::types::TradeEvent {
    persistence::types::TradeEvent { /* ... */ }
}
```

✅ Good (type aliases):
```rust
use parser::types::TradeEvent as ParserTradeEvent;
use persistence::types::TradeEvent as PersistenceTradeEvent;

fn convert(event: &ParserTradeEvent) -> PersistenceTradeEvent {
    PersistenceTradeEvent { /* ... */ }
}
```

## Type Alias Naming

- Type aliases that wrap external framework types should use explicit, prefixed names
- Avoid generic names like `Context` or `Command` that shadow well-known types
- Prefix with the framework name for clarity

❌ Bad:
```rust
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Command = poise::Command<Data, Error>;
```

✅ Good:
```rust
pub type PoiseContext<'a> = poise::Context<'a, Data, Error>;
pub type PoiseCommand = poise::Command<Data, Error>;
```

## Error Handling

- Use `Result<T, E>` for fallible operations
- Use `?` operator for error propagation
- Create meaningful error types with `thiserror` or `anyhow`
- Avoid `unwrap()` and `expect()` in production code

## Code Structure

- Keep modules small and focused
- Use `mod.rs` for public API definitions
- Document public APIs with `///` doc comments
- Implement traits where appropriate (Debug, Clone, etc.)

### File Naming Conventions

- Within a module subdirectory, do NOT prefix filenames with the module name
- The module directory already provides the namespace, so the prefix is redundant

❌ Bad:
```
src/wallet/wallet_service.rs
src/rugcheck/rugcheck_client.rs
```

✅ Good:
```
src/wallet/service.rs
src/rugcheck/client.rs
```

### File Size and Module Organization

- Keep files reasonably sized (aim for < 300-400 lines)
- Large files become hard to navigate and maintain
- Split large files into multiple smaller, focused modules
- Group related functionality into separate files
- Use subdirectories for related modules
- Always put type definitions in a dedicated `types.rs` file
- Always put error definitions in a dedicated `errors.rs` file
- Re-export public types and errors from `mod.rs`

Example structure:
```
src/
  feature/
    mod.rs          // Public API and re-exports
    types.rs        // Type definitions and structs
    errors.rs       // Error types
    core.rs         // Core functionality
    helpers.rs      // Helper functions
```

### Function Size and Indentation Depth

- Avoid deep nesting (max 3-4 levels)
- When exceeding 3-4 indentation levels: Extract logic into separate functions
- Large functions with many indentations should be split up
- Use early returns to reduce nesting
- Extract complex conditions into named functions

❌ Bad (too many indentations):
```rust
fn process() {
    if condition1 {
        if condition2 {
            if condition3 {
                // Deeply nested logic
            }
        }
    }
}
```

✅ Good (flat structure):
```rust
fn process() {
    if !condition1 {
        return;
    }
    if !condition2 {
        return;
    }
    
    handle_condition3();
}

fn handle_condition3() {
    // Extracted logic
}
```

## Binary Crate Structure (main.rs / app.rs Pattern)

- Keep `main.rs` thin: only bootstrap logic (dotenv, tracing, CLI parsing)
- Delegate all application logic to `app::run()` or `app::run_application()`
- This improves testability (`app::run()` can be called from integration tests) and separation of concerns

Example `main.rs`:
```rust
mod app;

#[tokio::main]
async fn main() {
    dotenvy::dotenv_override().ok();
    // ... tracing setup, CLI parsing ...

    if let Err(e) = app::run().await {
        tracing::error!("Application error: {}", e);
        std::process::exit(1);
    }
}
```

Example `app.rs`:
```rust
use anyhow::Result;

pub async fn run() -> Result<()> {
    // All application logic here
    Ok(())
}
```

## Performance & Ownership

- Use references (`&T`) instead of ownership where possible
- Avoid unnecessary clones
- Use `&str` instead of `String` for function parameters
- Prefer iterators over explicit loops

## Testing

- Write unit tests for critical logic
- Use `#[cfg(test)]` modules
- Test edge cases and error scenarios
- Always place tests in a separate `<module_name>.test.rs` file next to the source file
- Include the test file via `#[cfg(test)] #[path = "<module_name>.test.rs"] mod tests;` at the bottom of the source file
- Do NOT put `#[cfg(test)] mod tests { ... }` blocks inline in the source file

Example structure:
```
src/
  feature/
    trade_filter.rs          // Source code only, no inline tests
    trade_filter.test.rs     // All tests for trade_filter
```

Example inclusion in `trade_filter.rs`:
```rust
#[cfg(test)]
#[path = "trade_filter.test.rs"]
mod tests;
```
