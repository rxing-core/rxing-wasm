#[cfg(feature = "decode_hints")]
mod decode_hints;
mod encode_hints;

use std::collections::HashMap;

use encode_hints::EncodeHintDictionary;
use rxing::{self, ResultPoint};
use rxing::{Reader, Writer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// Available barcode types
pub enum BarcodeFormat {
    /** Aztec 2D barcode format. */
    AZTEC,

    /** CODABAR 1D format. */
    CODABAR,

    /** Code 39 1D format. */
    Code39,

    /** Code 93 1D format. */
    Code93,

    /** Code 128 1D format. */
    Code128,

    /** Data Matrix 2D barcode format. */
    DataMatrix,

    /** EAN-8 1D format. */
    Ean8,

    /** EAN-13 1D format. */
    Ean13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MAXICODE,

    /** PDF417 format. */
    Pdf417,

    /** QR Code 2D barcode format. */
    QrCode,

    /** RSS 14 */
    Rss14,

    /** RSS EXPANDED */
    RssExpanded,

    /** UPC-A 1D format. */
    UpcA,

    /** UPC-E 1D format. */
    UpcE,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UpcEanExtension,

    MicroQR,

    Telepen,

    RectangularMicroQR,

    ///
    UnsuportedFormat,
}

impl From<BarcodeFormat> for rxing::BarcodeFormat {
    fn from(value: BarcodeFormat) -> Self {
        match value {
            BarcodeFormat::AZTEC => rxing::BarcodeFormat::AZTEC,
            BarcodeFormat::CODABAR => rxing::BarcodeFormat::CODABAR,
            BarcodeFormat::Code39 => rxing::BarcodeFormat::CODE_39,
            BarcodeFormat::Code93 => rxing::BarcodeFormat::CODE_93,
            BarcodeFormat::Code128 => rxing::BarcodeFormat::CODE_128,
            BarcodeFormat::DataMatrix => rxing::BarcodeFormat::DATA_MATRIX,
            BarcodeFormat::Ean8 => rxing::BarcodeFormat::EAN_8,
            BarcodeFormat::Ean13 => rxing::BarcodeFormat::EAN_13,
            BarcodeFormat::ITF => rxing::BarcodeFormat::ITF,
            BarcodeFormat::MAXICODE => rxing::BarcodeFormat::MAXICODE,
            BarcodeFormat::Pdf417 => rxing::BarcodeFormat::PDF_417,
            BarcodeFormat::QrCode => rxing::BarcodeFormat::QR_CODE,
            BarcodeFormat::Rss14 => rxing::BarcodeFormat::RSS_14,
            BarcodeFormat::RssExpanded => rxing::BarcodeFormat::RSS_EXPANDED,
            BarcodeFormat::UpcA => rxing::BarcodeFormat::UPC_A,
            BarcodeFormat::UpcE => rxing::BarcodeFormat::UPC_E,
            BarcodeFormat::UpcEanExtension => rxing::BarcodeFormat::UPC_EAN_EXTENSION,
            BarcodeFormat::MicroQR => rxing::BarcodeFormat::MICRO_QR_CODE,
            BarcodeFormat::UnsuportedFormat => rxing::BarcodeFormat::UNSUPORTED_FORMAT,
            BarcodeFormat::Telepen => rxing::BarcodeFormat::TELEPEN,
            BarcodeFormat::RectangularMicroQR => rxing::BarcodeFormat::RECTANGULAR_MICRO_QR_CODE,
        }
    }
}

impl From<rxing::BarcodeFormat> for BarcodeFormat {
    fn from(value: rxing::BarcodeFormat) -> Self {
        match value {
            rxing::BarcodeFormat::AZTEC => BarcodeFormat::AZTEC,
            rxing::BarcodeFormat::CODABAR => BarcodeFormat::CODABAR,
            rxing::BarcodeFormat::CODE_39 => BarcodeFormat::Code39,
            rxing::BarcodeFormat::CODE_93 => BarcodeFormat::Code93,
            rxing::BarcodeFormat::CODE_128 => BarcodeFormat::Code128,
            rxing::BarcodeFormat::DATA_MATRIX => BarcodeFormat::DataMatrix,
            rxing::BarcodeFormat::EAN_8 => BarcodeFormat::Ean8,
            rxing::BarcodeFormat::EAN_13 => BarcodeFormat::Ean13,
            rxing::BarcodeFormat::ITF => BarcodeFormat::ITF,
            rxing::BarcodeFormat::MAXICODE => BarcodeFormat::MAXICODE,
            rxing::BarcodeFormat::PDF_417 => BarcodeFormat::Pdf417,
            rxing::BarcodeFormat::QR_CODE => BarcodeFormat::QrCode,
            rxing::BarcodeFormat::RSS_14 => BarcodeFormat::Rss14,
            rxing::BarcodeFormat::RSS_EXPANDED => BarcodeFormat::RssExpanded,
            rxing::BarcodeFormat::UPC_A => BarcodeFormat::UpcA,
            rxing::BarcodeFormat::UPC_E => BarcodeFormat::UpcE,
            rxing::BarcodeFormat::UPC_EAN_EXTENSION => BarcodeFormat::UpcEanExtension,
            rxing::BarcodeFormat::MICRO_QR_CODE => BarcodeFormat::MicroQR,
            rxing::BarcodeFormat::UNSUPORTED_FORMAT => BarcodeFormat::UnsuportedFormat,
            rxing::BarcodeFormat::TELEPEN => BarcodeFormat::Telepen,
            rxing::BarcodeFormat::RECTANGULAR_MICRO_QR_CODE => BarcodeFormat::RectangularMicroQR,
            _ => BarcodeFormat::UnsuportedFormat,
        }
    }
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct BarcodeResult {
    text: String,
    raw_bytes: Vec<u8>,
    num_bits: usize,
    result_points: Vec<f32>,
    format: BarcodeFormat,
    result_metadata: HashMap<String, String>,
    timestamp: isize,
}

#[wasm_bindgen]
impl BarcodeResult {
    pub fn timestamp(&self) -> isize {
        self.timestamp
    }

    pub fn format(&self) -> BarcodeFormat {
        self.format
    }

    /// Each pair of f32 values is an (x,y) point
    pub fn result_points(&self) -> Vec<f32> {
        self.result_points.to_vec()
    }

    pub fn num_bits(&self) -> usize {
        self.num_bits
    }

    pub fn raw_bytes(&self) -> Vec<u8> {
        self.raw_bytes.to_vec()
    }

    pub fn text(&self) -> String {
        self.text.to_owned()
    }

    pub fn get_meta_data(&self) -> js_sys::Map {
        let output_map = js_sys::Map::new();
        for (k, v) in &self.result_metadata {
            output_map.set(
                &wasm_bindgen::JsValue::from(v),
                &wasm_bindgen::JsValue::from(k),
            );
        }
        output_map
    }
}

fn get_result_metadata_name(mdtn: &rxing::RXingResultMetadataType) -> String {
    match mdtn {
        rxing::RXingResultMetadataType::OTHER => "OTHER",
        rxing::RXingResultMetadataType::ORIENTATION => "Orientation",
        rxing::RXingResultMetadataType::BYTE_SEGMENTS => "Byte_Segments",
        rxing::RXingResultMetadataType::ERROR_CORRECTION_LEVEL => "Error_Correction_Level",
        rxing::RXingResultMetadataType::ISSUE_NUMBER => "Issue_Number",
        rxing::RXingResultMetadataType::SUGGESTED_PRICE => "Suggested_Price",
        rxing::RXingResultMetadataType::POSSIBLE_COUNTRY => "Possible_Country",
        rxing::RXingResultMetadataType::UPC_EAN_EXTENSION => "UPC/EAN_Extension",
        rxing::RXingResultMetadataType::PDF417_EXTRA_METADATA => "PDF417_Extra_MetaData",
        rxing::RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE => "Structured_Append_Sequence",
        rxing::RXingResultMetadataType::STRUCTURED_APPEND_PARITY => "Structured_Append_Parity",
        rxing::RXingResultMetadataType::SYMBOLOGY_IDENTIFIER => "Symbology_Identifier",
        rxing::RXingResultMetadataType::IS_MIRRORED => "Is_Mirrored",
        rxing::RXingResultMetadataType::CONTENT_TYPE => "Content_Type",
        rxing::RXingResultMetadataType::IS_INVERTED => "Is_Inverted",
    }
    .to_owned()
}

fn get_result_metadata_value(res_mdt_val: &rxing::RXingResultMetadataValue) -> String {
    match res_mdt_val {
        rxing::RXingResultMetadataValue::OTHER(v)
        | rxing::RXingResultMetadataValue::SuggestedPrice(v)
        | rxing::RXingResultMetadataValue::PossibleCountry(v)
        | rxing::RXingResultMetadataValue::UpcEanExtension(v)
        | rxing::RXingResultMetadataValue::SymbologyIdentifier(v)
        | rxing::RXingResultMetadataValue::ContentType(v)
        | rxing::RXingResultMetadataValue::ErrorCorrectionLevel(v) => v.to_owned(),

        rxing::RXingResultMetadataValue::Orientation(v)
        | rxing::RXingResultMetadataValue::IssueNumber(v)
        | rxing::RXingResultMetadataValue::StructuredAppendSequence(v)
        | rxing::RXingResultMetadataValue::StructuredAppendParity(v) => v.to_string(),

        rxing::RXingResultMetadataValue::ByteSegments(v) => format!("{v:?}"),

        rxing::RXingResultMetadataValue::Pdf417ExtraMetadata(v) => format!("{v:?}"),

        rxing::RXingResultMetadataValue::IsMirrored(v) => v.to_string(),
        rxing::RXingResultMetadataValue::IsInverted(v) => v.to_string(),
    }
}

impl From<rxing::RXingResult> for BarcodeResult {
    fn from(value: rxing::RXingResult) -> Self {
        // let meta_data = value.getRXingResultMetadata();
        // let meta_data_rebuild = meta_data.iter().map(|(k,v)| (k.to_string(), v.to_string())).collect();
        Self {
            text: value.getText().to_owned(),
            raw_bytes: value.getRawBytes().to_vec(),
            num_bits: value.getNumBits(),
            result_points: value
                .getRXingResultPoints()
                .iter()
                .flat_map(|rxp| [rxp.getX(), rxp.getY()])
                .collect(),
            format: value.getBarcodeFormat().to_owned().into(),
            result_metadata: value
                .getRXingResultMetadata()
                .iter()
                .map(|(k, v)| (get_result_metadata_name(k), get_result_metadata_value(v)))
                .collect::<HashMap<String, String>>(),
            timestamp: value.getTimestamp() as isize,
        }
    }
}

#[wasm_bindgen]
/// Encode a barcode with the given data, dimensions, and type
pub fn encode_barcode(
    data: &str,
    width: u32,
    height: u32,
    bc_type: BarcodeFormat,
) -> Result<String, String> {
    let writer = rxing::MultiFormatWriter::default();
    let Ok(bit_matrix) = writer.encode_with_hints(
        data,
        &bc_type.into(),
        width as i32,
        height as i32,
        &std::collections::HashMap::new(),
    ) else {
        return Err("couldn't encode".to_owned());
    };
    Ok(bit_matrix.to_string())
}

#[wasm_bindgen]
/// Encode a barcode with the given data, dimensions, and type, use the given encoding hints
pub fn encode_barcode_with_hints(
    data: &str,
    width: u32,
    height: u32,
    bc_type: BarcodeFormat,
    hints: &mut EncodeHintDictionary,
) -> Result<String, String> {
    let writer = rxing::MultiFormatWriter::default();
    let Ok(bit_matrix) = writer.encode_with_hints(
        data,
        &bc_type.into(),
        width as i32,
        height as i32,
        hints.get_dictionary_mut(),
    ) else {
        return Err("couldn't encode".to_owned());
    };
    Ok(bit_matrix.to_string())
}

#[wasm_bindgen]
/// Decode a barcode from an array of 8bit luma data
pub fn decode_barcode(
    data: Vec<u8>,
    width: u32,
    height: u32,
    try_harder: Option<bool>,
    filter_image: Option<bool>,
) -> Result<BarcodeResult, String> {
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    if let Some(true) = try_harder {
        hints.insert(
            rxing::DecodeHintType::TRY_HARDER,
            rxing::DecodeHintValue::TryHarder(true),
        );
    }

    let detection_function = if matches!(filter_image, Some(true)) {
        rxing::helpers::detect_in_luma_filtered_with_hints
    }else {
        rxing::helpers::detect_in_luma_with_hints
    };

    let Ok(result) = 
        detection_function(data, width, height, None, &mut hints)
    else {
        return Err("not found".to_owned());
    };
    Ok(result.into())
}

#[wasm_bindgen]
/// Convert a javascript image context's data into luma 8.
///
/// Data for this function can be found from any canvas object
/// using the `data` property of an `ImageData` object.
/// Such an object could be obtained using the `getImageData`
/// method of a `CanvasRenderingContext2D` object.
pub fn convert_js_image_to_luma(data: &[u8]) -> Vec<u8> {
    let mut luma_data = Vec::with_capacity(data.len() / 4);
    for src_pixel in data.chunks_exact(4) {
        let [red, green, blue, alpha] = src_pixel else {
            continue;
        };
        let pixel = if *alpha == 0 {
            // white, so we know its luminance is 255
            0xFF
        } else {
            // .299R + 0.587G + 0.114B (YUV/YIQ for PAL and NTSC),
            // (306*R) >> 10 is approximately equal to R*0.299, and so on.
            // 0x200 >> 10 is 0.5, it implements rounding.

            ((306 * (*red as u64) + 601 * (*green as u64) + 117 * (*blue as u64) + 0x200) >> 10)
                as u8
        };
        luma_data.push(pixel);
    }

    luma_data
}

#[wasm_bindgen]
/// Decode a barcode from an array of rgba data.
/// Pixel data is in the form of:
///     Each pixel is one u32, [r,g,b].
pub fn decode_barcode_rgb(
    data: Vec<u32>,
    width: u32,
    height: u32,
    try_harder: Option<bool>,
) -> Result<BarcodeResult, String> {
    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    if let Some(true) = try_harder {
        hints.insert(
            rxing::DecodeHintType::TRY_HARDER,
            rxing::DecodeHintValue::TryHarder(true),
        );
    }

    let mut multi_format_reader = rxing::MultiFormatReader::default();

    let Ok(result) = multi_format_reader.decode_with_hints(
        &mut rxing::BinaryBitmap::new(rxing::common::HybridBinarizer::new(
            rxing::RGBLuminanceSource::new_with_width_height_pixels(
                width as usize,
                height as usize,
                &data,
            ),
        )),
        &hints,
    ) else {
        return Err("not found".to_owned());
    };

    Ok(result.into())
}

#[cfg(feature = "decode_hints")]
#[wasm_bindgen]
pub fn decode_barcode_with_hints(
    data: Vec<u8>,
    width: u32,
    height: u32,
    hints: &mut decode_hints::DecodeHintDictionary,
    filter_image: Option<bool>, 
) -> Result<BarcodeResult, String> {

    let results = if matches!(filter_image, Some(true)) {
        rxing::helpers::detect_in_luma_filtered_with_hints(
            data,
            width,
            height,
            None,
            hints.get_dictionary_mut(),
        )
    }else {
        rxing::helpers::detect_in_luma_with_hints(
            data,
            width,
            height,
            None,
            hints.get_dictionary_mut(),
        )
    };
    
    let Ok(result) = results else {
        return Err("not found".to_owned());
    };
    Ok(result.into())
}

#[wasm_bindgen]
pub struct MultiDecodeResult {
    pointer: usize,
    results: Vec<BarcodeResult>,
}

#[wasm_bindgen]
impl MultiDecodeResult {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MultiDecodeResult {
        MultiDecodeResult {
            pointer: 0,
            results: Vec::default(),
        }
    }

    fn with_results(results: Vec<BarcodeResult>) -> MultiDecodeResult {
        MultiDecodeResult {
            pointer: 0,
            results,
        }
    }

    #[wasm_bindgen]
    pub fn next(&mut self) -> Option<BarcodeResult> {
        let ret = self.results.get(self.pointer).map(|b| b.clone());
        self.pointer += 1;
        ret
    }
}

#[cfg(feature = "decode_hints")]
#[wasm_bindgen]
pub fn decode_multi(
    data: Vec<u8>,
    width: u32,
    height: u32,
    hints: &mut decode_hints::DecodeHintDictionary,
) -> Result<MultiDecodeResult, String> {
    let Ok(results) = rxing::helpers::detect_multiple_in_luma_with_hints(
        data,
        width,
        height,
        hints.get_dictionary_mut(),
    ) else {
        return Err("not found".to_owned());
    };
    Ok(MultiDecodeResult::with_results(
        results.into_iter().map(|r| r.into()).collect(),
    ))
}
