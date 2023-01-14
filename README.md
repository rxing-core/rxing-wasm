# rxing-wasm
WASM bindings for common rxing functions

## Functions
```rust
pub fn encode_barcode(
    data: &str,
    width: u32,
    height: u32,
    bc_type: BarcodeFormat,
) -> Result<String, String>;
```

```rust
pub fn decode_barcode(
    data: Vec<u8>,
    width: u32,
    height: u32,
    try_harder: Option<bool>,
) -> Result<BarcodeResult, String>;
```