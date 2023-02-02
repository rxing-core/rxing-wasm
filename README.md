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

## Hints
### Using the `DecodeHintDictionary` class
Add a hint with `set_hint(hint: DecodeHintTypes, value: string)`. The function returns `true` if the hint was added and `false` if it was not. The value of hint must be a `number` representing on of the enum values for `DecodeHintTypes`. The easiest way to use this is to simply pass in one of the values from `DecodeHintTypes`.

Remove a hint using `remove_hint(hint: DecodeHintTypes)`. The function returns `true` if the hint was present and removed, and `false` otherwise. The hint value is the same as in the add_hint function.

### `DecodeHintTypes` Values
The following values are available for the `DecodeHintTypes` enum.
* `Other`: Unspecified, application-specific hint. This can be any string, only some decoders care about this value.
* `PureBarcode`: Image is a pure monochrome image of a barcode. Should be a string with either "true" or "false".
* `PossibleFormats`: A comma separated list (no spaces between elements), with the names of barcode formats to search for. For example, looking for a QrCode and a Datamatrix, one would pass in the string "qrcode,datamatrix".
* `TryHarder`: Spend more time to try to find a barcode; optimize for accuracy, not speed. String with either "true" or "false".
* `CharacterSet`: Specifies what character encoding to use when decoding, where applicable. This should be a string mapping to a character encoding.
* `AllowedLengths`: Allowed lengths of encoded data -- reject anything else. A comma separated list of integers.
* `AssumeCode39CheckDigit`: Assume Code 39 codes employ a check digit. A string with either "true" or "false".
* `AssumeGs1`: Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed. For example this affects FNC1 handling for Code 128 (aka GS1-128). A string with either "true" or "false".
* `ReturnCodabarStartEnd`: If true, return the start and end digits in a Codabar barcode instead of stripping them. They are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them to not be. A string with either "true" or "false".
* `NeedResultPointCallback`: The caller needs to be notified via callback when a possible ResultPoint is found. Currently unsupported. In future will map to a (x: number, y: number) = {} function.
* `AllowedEanExtensions`: Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this. A comma separated list of the allowed extension lengths, for example "2", "5" or "2,5". If it is optional to have an extension, do not set this hint. If this is set, and a UPC or EAN barcode is found but an extension is not, then no result will be returned at all.
* `AlsoInverted`: If true, also tries to decode as inverted image. All configured decoders are simply called a second time with an inverted image. A string with either "true" or "false".

## Result Metadata
Result metadata is now available through the `get_result_metadata_name` method of the `BarcodeResult` class. The returned result is a javascript `Map` object representing availble decoded metadata. The possible keys are:
* `OTHER`
* `Orientation`
* `Byte_Segments`
* `Error_Correction_Level`
* `Issue_Number`
* `Suggested_Price`
* `Possible_Country`
* `UPC/EAN_Extension`
* `PDF417_Extra_MetaData`
* `Structured_Append_Sequence`
* `Structured_Append_Parity`
* `Symbology_Identifier`

It is important to note that not all values will be set for all results.

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