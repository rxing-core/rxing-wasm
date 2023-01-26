# rxing-wasm
WASM bindings for common rxing functions. The NPM link is [https://www.npmjs.com/package/rxing-wasm](https://www.npmjs.com/package/rxing-wasm) and the rust source is [https://github.com/hschimke/rxing-wasm](https://github.com/hschimke/rxing-wasm).

## Data
The `convert_js_image_to_luma` function is used to convert canvas image data to the luma 8
format that rxing expects. An example might look like to below.

```javascript
function decodeBarcode(canvas) {
    let context = canvas.getContext('2d');
    let height = canvas.height;
    let width = canvas.width;
    let imageData = context.getImageData(0, 0, width, height);

    let data = imageData.data;
    let luma8Data = convert_js_image_to_luma(data);
    let parsedBarcode = decode_barcode(luma8Data, width, height);
    
    return parsedBarcode;
}
```

## Functions
```rust
pub fn convert_js_image_to_luma(data: &[u8]) -> Vec<u8>;
```

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

```rust
pub fn decode_barcode_with_hints(
    data: Vec<u8>,
    width: u32,
    height: u32,
    hints: &mut decode_hints::DecodeHintDictionary,
) -> Result<BarcodeResult, String>;
```