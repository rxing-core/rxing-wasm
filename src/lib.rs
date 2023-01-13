use rxing;
use wasm_bindgen::prelude::*;
use rxing::Writer;

#[wasm_bindgen]
pub fn encode_barcode(
    data: &str,
    width: u32,
    height: u32,
    bc_type: &str,
) -> Result<String, String> {
    let writer = rxing::MultiFormatWriter::default();
    let Ok(bit_matrix) = writer.encode_with_hints(
        data,
        &bc_type.into(),
        width as i32,
        height as i32,
        &std::collections::HashMap::new(),
    ) else { return Err("couldn't encode".to_owned()) };
    Ok(bit_matrix.to_string())
}

#[wasm_bindgen]
pub fn decode_barcode(data:Vec<u8>, width: u32, height: u32) -> Result<String,String> {
    let Ok(result) = rxing::helpers::detect_in_luma(data, width, height, None) else {
        return Err("not found".to_owned());
    };
    Ok(result.getText().to_owned())
}