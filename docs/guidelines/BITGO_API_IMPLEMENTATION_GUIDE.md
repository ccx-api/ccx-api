# BitGo API Implementation Guide

This guide provides step-by-step instructions for adding new API methods to the `ccx-bitgo` crate, following established patterns and best practices.

## Table of Contents
- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Step 1: Gather API Documentation](#step-1-gather-api-documentation)
- [Step 2: Determine File Structure](#step-2-determine-file-structure)
- [Step 3: Create the API Method](#step-3-create-the-api-method)
- [Step 4: Handle Shared Types](#step-4-handle-shared-types)
- [Step 5: Update Module Exports](#step-5-update-module-exports)
- [Step 6: Add Unit Tests](#step-6-add-unit-tests)
- [Step 7: Add Usage Examples](#step-7-add-usage-examples)
- [Step 8: Verification](#step-8-verification)
- [Implementation Checklist](#implementation-checklist)

## Overview

This guide helps you implement new BitGo API endpoints by following the established patterns in the `ccx-bitgo` crate. We use macros for automatic serialization/deserialization and builder pattern generation.

## Prerequisites

- Rust development environment
- Access to official BitGo API documentation
- Understanding of the target API endpoint's functionality

## Step 1: Gather API Documentation

**⚠️ CRITICAL: Always use official BitGo API documentation. Never guess parameters or response fields.**

1. **Obtain the official API documentation URL** from the user
2. **Ask the user to provide the exact query parameters** available for the endpoint
3. **Request sample request/response examples** if available
4. **Verify the HTTP method** (GET, POST, PUT, DELETE)
5. **Confirm the endpoint URL structure** (e.g., `/api/v2/{coin}/wallet/{id}`)

### Example Questions to Ask:
```markdown
Please provide:
1. The official BitGo API documentation URL
2. All available query parameters with their types and descriptions
3. Sample JSON response from the API
4. The HTTP method used
5. The exact endpoint URL structure
```

## Step 2: Determine File Structure

### File Naming Convention
- Use descriptive names that reflect the method's functionality
- Format: `{action}_{resource}_by_{identifier}.rs`
- Examples:
  - `get_wallet_by_id_coin.rs` - Get wallet by ID and coin
  - `list_transactions_by_wallet.rs` - List transactions for a wallet
  - `create_wallet_address.rs` - Create new wallet address

### Directory Structure
```
crates/ccx-bitgo/src/api/
├── wallet.rs           # Module file containing submodule declarations
├── wallet/
│   ├── get_wallet_by_id_coin.rs
│   ├── list_wallets.rs
│   └── total_balances.rs
├── transaction.rs      # Module file for transaction endpoints
├── transaction/
│   └── ...
└── mod.rs             # Root API module
```

## Step 3: Create the API Method

### Basic Template

Create a new file following this template:

```rust
use std::borrow::Cow;

use macro_rules_attribute::apply;

use crate::prelude::{BaseAmount, Coin};
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

/// Request to [describe the API method functionality]
#[apply(Request)]
pub struct YourMethodName {
    // Path parameters (always marked with #[serde(skip)])
    #[serde(skip)]
    coin: Coin,

    #[serde(skip)]
    some_id: String,

    // Query parameters (only include those documented by BitGo)
    /// Description from official docs
    parameter_name: Option<DataType>,

    /// Another parameter description
    another_parameter: Option<bool>,

    /// Amount in smallest units (use when API expects string amounts)
    amount: Option<BaseAmount>,
}

/// Response for YourMethodName request
#[apply(Response)]
pub struct YourMethodNameResponse {
    /// Transaction ID
    pub id: String,
    /// Transaction status
    pub status: String,
    /// Amount in smallest units (when API returns string amounts)
    pub amount: BaseAmount,
    /// Fee paid in smallest units
    pub fee: Option<BaseAmount>,
    /// Confirmation count
    pub confirmations: u32,
}

impl Response for YourMethodNameResponse {}

impl Request for YourMethodName {
    type Response = YourMethodNameResponse;

    const HTTP_METHOD: http::Method = http::Method::GET; // or POST, PUT, DELETE

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> Cow<'static, str> {
        // Build the path using path parameters
        let coin = &self.coin;
        let some_id = &self.some_id;

        format!("/api/v2/{coin}/some-endpoint/{some_id}").into()
    }
}

impl SignedRequest for YourMethodName {}
```

### Key Implementation Notes

1. **Path Parameters**: Always mark with `#[serde(skip)]` and use in `path()` method
2. **Query Parameters, Request Body, and Response**: Let the macro handle camelCase conversion automatically
3. **Builder Pattern**: The `apply(Request)` macro automatically handles conversions:
   - `Option<T>` types - you can pass values directly without wrapping in `Some()`
   - Type conversions - no need for explicit `.into()`, `.to_string()`, etc.
   - The builder accepts compatible types and converts them automatically
4. **HTTP Method**: Use the correct method as documented in the API
5. **Rate Limiting**: Use `RateLimitType::Authenticated` for most endpoints
6. **Response Types**: Reuse existing types when possible, define new ones when needed

### Working with BaseAmount for Smallest Units

BitGo APIs often return cryptocurrency amounts in their smallest units (satoshis, wei, etc.) as strings to maintain precision. Use the `BaseAmount` type for these values.

**When to use BaseAmount:**
- API documentation shows amounts as strings representing smallest units
- Example: `{"amount": "1000000"}` (string) → use `BaseAmount`
- Counter-example: `{"confirmations": 6}` (number) → use `u32`

**Setting BaseAmount fields in requests:**
```rust
use crate::types::BaseAmount;

#[apply(Request)]
pub struct SendRequest {
    #[serde(skip)]
    wallet_id: String,

    #[builder(into)]
    amount: BaseAmount,              // Pass i128 directly to builder
    #[builder(into)]
    fee_rate: Option<BaseAmount>,    // Optional BaseAmount field
}

// Usage - pass i128 values directly
let request = SendRequest::builder()
    .wallet_id("wallet123")
    .amount(50_000_000i128)          // Auto-converts to BaseAmount
    .fee_rate(2000i128)              // Optional field
    .build();
```

## Step 4: Handle Shared Types

### When to Create New Types
- Response contains complex nested objects
- Multiple endpoints use the same data structure
- Type will be reused across different API methods

### Special Case: Using Existing Types Directly
In rare cases where an API endpoint returns exactly the structure of an existing type (e.g., `GetWalletById` returns a `Wallet`), you can skip the response wrapper:

```rust
impl Request for GetWalletById {
    type Response = Wallet; // Direct usage for exact matches
    // ...
}
```

**Requirements for direct usage:**
- API response structure exactly matches existing type
- Existing type implements `crate::proto::Response`
- No additional wrapper fields needed

### Where to Place Shared Types
```
crates/ccx-bitgo/src/types/
├── wallet.rs      # Wallet-related types
├── transaction.rs # Transaction-related types
├── coin.rs        # Coin type definitions
└── (types.rs in parent contains mod declarations)
```

### Creating Shared Types
```rust
// In crates/ccx-bitgo/src/types/your_module.rs
use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};

use crate::types::derive::Response;

#[apply(Response)]
pub struct SharedDataType {
    pub field1: String,
    pub field2: Option<u64>,
    // ... other fields
}
```

### Exposing New Types
```rust
// In crates/ccx-bitgo/src/types.rs
pub mod your_module;

// In crates/ccx-bitgo/src/prelude.rs
pub use crate::types::your_module::*;
```

### Re-exporting Types from Parent Modules
When extracting shared types to the `types/` directory, you should also re-export these types from the corresponding parent API module. This makes the types easily accessible through the module they're most commonly used with.

```rust
// In crates/ccx-bitgo/src/api/transfer.rs
mod fee_estimate;
mod get_transfer;
mod list_transfers;

pub use fee_estimate::*;
pub use get_transfer::*;
pub use list_transfers::*;
// Re-export shared transfer types for convenience
pub use crate::types::transfer::*;
```

This pattern allows users to access types in multiple ways:
- Through the prelude: `use ccx_bitgo::prelude::*;` (gets everything)
- Through the specific module: `use ccx_bitgo::api::transfer::TransferState;`
- Through the types module: `use ccx_bitgo::types::transfer::TransferState;`

This provides flexibility while maintaining clear organization.

## Step 5: Update Module Exports

### Update API Module
```rust
// In crates/ccx-bitgo/src/api/wallet.rs
mod get_wallet_by_id_coin;
mod list_wallets;
mod your_new_method;

pub use get_wallet_by_id_coin::*;
pub use list_wallets::*;
pub use your_new_method::*;
```

### Update Parent API Module
```rust
// In crates/ccx-bitgo/src/api/mod.rs
pub mod wallet;
// ... other modules
```

## Step 6: Add Unit Tests

Create focused tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    mod request_serialization {
        use super::*;

        #[test]
        fn test_basic_request_no_parameters() {
            let request = YourMethodName::builder()
                .coin("btc")
                .some_id("test_id")
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("Failed to parse");

            // Should be empty for requests with no query parameters
            assert_eq!(parsed, serde_json::json!({}));
        }

        #[test]
        fn test_request_with_parameters() {
            let request = YourMethodName::builder()
                .coin("btc")
                .some_id("test_id")
                .parameter_name("value")
                .another_parameter(true)
                .amount(100_000_000i128)  // BaseAmount field
                .build();

            let serialized = serde_json::to_string(&request).expect("Failed to serialize");
            let parsed: serde_json::Value = serde_json::from_str(&serialized).expect("Failed to parse");

            let expected = serde_json::json!({
                "parameterName": "value",
                "anotherParameter": true,
                "amount": "100000000"  // BaseAmount serializes as string
            });

            assert_eq!(parsed, expected);
        }
    }

    mod response_deserialization {
        use super::*;

        #[test]
        fn test_successful_response() {
            // Use actual response format from BitGo API documentation
            let response_json = r#"{
                "id": "abc123",
                "status": "confirmed",
                "amount": "1000000",
                "fee": "5000",
                "confirmations": 6
            }"#;

            let response: YourMethodNameResponse = serde_json::from_str(response_json)
                .expect("Failed to deserialize response");

            assert_eq!(response.id, "abc123");
            assert_eq!(response.status, "confirmed");
            assert_eq!(response.amount, BaseAmount::from(1000000i128));
            assert_eq!(response.fee, Some(BaseAmount::from(5000i128)));
            assert_eq!(response.confirmations, 6);
        }
    }

    mod path_generation {
        use super::*;

        #[test]
        fn test_path_construction() {
            let request = YourMethodName::builder()
                .coin("btc")
                .some_id("wallet123")
                .build();

            let path = request.path();
            assert_eq!(path, "/api/v2/btc/some-endpoint/wallet123");
        }
    }
}
```

### Test Organization Guidelines
- **`request_serialization`**: Test query parameter serialization
- **`response_deserialization`**: Test API response parsing
- **`path_generation`**: Test URL construction

**Important**: When testing with the builder pattern, pass values directly without explicit conversions like `Some()`, `.into()`, or `.to_string()`. The `#[apply(Request)]` macro automatically handles all necessary type conversions.

## Step 7: Add Usage Examples

### Update Examples File
Add usage example to `crates/ccx-bitgo/examples/` (appropriate category):

```rust
// In the relevant example file (e.g., wallet.rs)

// Example: [Describe what this demonstrates]
println!("📝 [Action description]...");
let result = YourMethodName::builder()
    .coin("btc")
    .some_id(config.some_id.clone())
    .parameter_name("example_value")
    .another_parameter(true)
    .amount(50_000_000i128)  // BaseAmount field - 0.5 BTC in satoshis
    .build()
    .throttle(&rate_limiter)
    .sign_now_and_send(&credential, &client)
    .await?
    .into_payload()?;

dbg!(&result);
```

## Step 8: Verification

### Compile Check
```bash
cd crates/ccx-bitgo
cargo check
```

### Run Tests
```bash
# Run specific method tests
cargo test your_method_name::tests

# Run all tests
cargo test
```

### Example Verification
```bash
# Test examples compile
cargo check --examples
```

## Implementation Checklist

### 📋 Pre-Implementation
- [ ] Obtained official BitGo API documentation URL
- [ ] Confirmed all available query parameters with user
- [ ] Verified HTTP method and endpoint structure
- [ ] Received sample request/response examples
- [ ] Determined appropriate file name and location

### 🏗️ Implementation
- [ ] Created new file with descriptive name (`{action}_{resource}_by_{identifier}.rs`)
- [ ] Implemented request struct with `#[apply(Request)]`
- [ ] Marked path parameters with `#[serde(skip)]`
- [ ] Added only documented query parameters (no guessing)
- [ ] Implemented response struct with appropriate types
- [ ] Used existing shared types where applicable
- [ ] Implemented `Request` trait with correct HTTP method
- [ ] Implemented `SignedRequest` trait
- [ ] Created proper `path()` method using path parameters

### 📦 Module Integration
- [ ] Updated module exports in parent directory
- [ ] Used modern module pattern (`wallet.rs` not `wallet/mod.rs`)
- [ ] Added new types to `types/` directory if needed
- [ ] Updated `types.rs` to include new types module
- [ ] Updated `prelude.rs` to export new types
- [ ] Re-exported shared types from parent API modules for convenience
- [ ] Verified no circular dependencies

### 🧪 Testing
- [ ] Added `request_serialization` tests
- [ ] Added `response_deserialization` tests
- [ ] Added `path_generation` tests
- [ ] Used realistic test data from API documentation
- [ ] Verified all tests pass (`cargo test`)
- [ ] Tests cover edge cases and different parameter combinations

### 📚 Documentation & Examples
- [ ] Added usage example to appropriate example file
- [ ] Example demonstrates realistic use case
- [ ] Example includes error handling
- [ ] Example shows parameter usage
- [ ] Verified examples compile (`cargo check --examples`)

### ✅ Final Verification
- [ ] Code compiles without warnings (`cargo check`)
- [ ] All tests pass (`cargo test`)
- [ ] Examples compile and run
- [ ] No duplicate code
- [ ] Follows established naming conventions
- [ ] Implementation matches official API documentation exactly
- [ ] No hardcoded or guessed parameters

### 🚀 Ready for Use
- [ ] Implementation follows crate patterns
- [ ] Tests provide adequate coverage
- [ ] Code is well-documented
- [ ] Examples demonstrate proper usage
- [ ] Ready for integration with BitGo API

---

## Common Patterns Reference

### Request Struct Pattern
```rust
#[apply(Request)]
pub struct MethodName {
    #[serde(skip)]
    path_param: String,

    query_param: Option<Type>,
}
```

### Response Struct Pattern
```rust
#[apply(Response)]
pub struct MethodNameResponse {
    /// Response field descriptions from API docs
    pub field1: String,
    pub field2: Option<u64>,
    pub field3: bool,
}
```

### Test Module Pattern
```rust
#[cfg(test)]
mod tests {
    mod request_serialization { /* ... */ }
    mod response_deserialization { /* ... */ }
    mod path_generation { /* ... */ }
}
```

This guide ensures consistent, well-tested implementations that follow BitGo API specifications exactly.
