use std::collections::HashMap;

use rxing::Writer;
use rxing::{self, ResultPoint};
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
            BarcodeFormat::UnsuportedFormat => rxing::BarcodeFormat::UNSUPORTED_FORMAT,
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
            rxing::BarcodeFormat::UNSUPORTED_FORMAT => BarcodeFormat::UnsuportedFormat,
        }
    }
}

#[wasm_bindgen]
pub struct BarcodeResult {
     text: String,
     raw_bytes: Vec<u8>,
     num_bits: usize,
     result_points: Vec<f32>,
     format: BarcodeFormat,
    // resultMetadata: HashMap<String, String>,
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

    pub fn raw_bytes(&self) ->Vec<u8> {
        self.raw_bytes.to_vec()
    }

    pub fn text(&self) -> String {
        self.text.to_owned()
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
                .map(|rxp| 
                 [    rxp.getX(),
                     rxp.getY(),
                ])
                .flatten()
                .collect(),
            format: value.getBarcodeFormat().to_owned().into(),
            // resultMetadata: value.getRXingResultMetadata(),
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
    ) else { return Err("couldn't encode".to_owned()) };
    Ok(bit_matrix.to_string())
}

#[wasm_bindgen]
/// Decode a barcode from an array of 8bit luma data
pub fn decode_barcode(
    data: Vec<u8>,
    width: u32,
    height: u32,
    try_harder: Option<bool>,
) -> Result<BarcodeResult, String> {
    let mut hints : rxing::DecodingHintDictionary = HashMap::new();
    if let Some(true) = try_harder {
        hints.insert(rxing::DecodeHintType::TRY_HARDER, rxing::DecodeHintValue::TryHarder(true));
    }
    let Ok(result) = rxing::helpers::detect_in_luma_with_hints(data, width, height, None, &mut hints) else {
        return Err("not found".to_owned());
    };
    Ok(result.into())
}
