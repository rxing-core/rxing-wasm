use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum DecodeHintTypes {
    /**
     * Unspecified, application-specific hint. Maps to an unspecified {@link Object}.
     */
    Other,

    /**
     * Image is a pure monochrome image of a barcode. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    PureBarcode,

    /**
     * Image is known to be of one of a few possible formats.
     * Maps to a {@link List} of {@link BarcodeFormat}s.
     */
    PossibleFormats,

    /**
     * Spend more time to try to find a barcode; optimize for accuracy, not speed.
     * Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    TryHarder,

    /**
     * Specifies what character encoding to use when decoding, where applicable (type String)
     */
    CharacterSet,

    /**
     * Allowed lengths of encoded data -- reject anything else. Maps to an {@code int[]}.
     */
    AllowedLengths,

    /**
     * Assume Code 39 codes employ a check digit. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeCode39CheckDigit,

    /**
     * Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
     * For example this affects FNC1 handling for Code 128 (aka GS1-128). Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeGs1,

    /**
     * If true, return the start and end digits in a Codabar barcode instead of stripping them. They
     * are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
     * to not be. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    ReturnCodabarStartEnd,

    /**
     * The caller needs to be notified via callback when a possible {@link RXingResultPoint}
     * is found. Maps to a {@link RXingResultPointCallback}.
     */
    NeedResultPointCallback,

    /**
     * Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
     * Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
     * If it is optional to have an extension, do not set this hint. If this is set,
     * and a UPC or EAN barcode is found but an extension is not, then no result will be returned
     * at all.
     */
    AllowedEanExtensions,

    /**
     * If true, also tries to decode as inverted image. All configured decoders are simply called a
     * second time with an inverted image. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    AlsoInverted,
}

impl From<DecodeHintTypes> for rxing::DecodeHintType {
    fn from(value: DecodeHintTypes) -> Self {
        match value {
            DecodeHintTypes::Other => rxing::DecodeHintType::OTHER,
            DecodeHintTypes::PureBarcode => rxing::DecodeHintType::PURE_BARCODE,
            DecodeHintTypes::PossibleFormats => rxing::DecodeHintType::POSSIBLE_FORMATS,
            DecodeHintTypes::TryHarder => rxing::DecodeHintType::TRY_HARDER,
            DecodeHintTypes::CharacterSet => rxing::DecodeHintType::CHARACTER_SET,
            DecodeHintTypes::AllowedLengths => rxing::DecodeHintType::ALLOWED_LENGTHS,
            DecodeHintTypes::AssumeCode39CheckDigit => {
                rxing::DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT
            }
            DecodeHintTypes::AssumeGs1 => rxing::DecodeHintType::ASSUME_GS1,
            DecodeHintTypes::ReturnCodabarStartEnd => {
                rxing::DecodeHintType::RETURN_CODABAR_START_END
            }
            DecodeHintTypes::NeedResultPointCallback => {
                rxing::DecodeHintType::NEED_RESULT_POINT_CALLBACK
            }
            DecodeHintTypes::AllowedEanExtensions => rxing::DecodeHintType::ALLOWED_EAN_EXTENSIONS,
            DecodeHintTypes::AlsoInverted => rxing::DecodeHintType::ALSO_INVERTED,
        }
    }
}

#[wasm_bindgen]
pub struct DecodeHintDictionary(HashMap<rxing::DecodeHintType, rxing::DecodeHintValue>);

#[wasm_bindgen]
impl DecodeHintDictionary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DecodeHintDictionary {
        DecodeHintDictionary(HashMap::new())
    }

    pub fn get_hint(&self, hint: DecodeHintTypes) -> String {
        if let Some(val) = self.0.get(&hint.into()) {
            match val {
                _ => todo!(),
            }
        } else {
            "".to_owned()
        }
    }

    pub fn set_hint(&self, hint: DecodeHintTypes, value: &str) -> bool {
        todo!()
    }

    pub fn remove_hint(&self, hint: DecodeHintTypes) -> bool {
        todo!()
    }
}

impl DecodeHintDictionary {
    pub fn get_dictionary(&self) -> &HashMap<rxing::DecodeHintType, rxing::DecodeHintValue> {
        &self.0
    }
}
