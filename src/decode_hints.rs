use rxing::DecodeHints;
use std::collections::HashSet;
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

    /**
     * Translate the ASCII values parsed by the Telepen reader into the Telepen Numeric form; use {@link Boolean#TRUE}.
     */
    TelepenAsNumeric,
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
            DecodeHintTypes::TelepenAsNumeric => rxing::DecodeHintType::TELEPEN_AS_NUMERIC,
        }
    }
}

#[wasm_bindgen]
#[derive(Default, Clone)]
pub struct DecodeHintDictionary(DecodeHints);

#[wasm_bindgen]
impl DecodeHintDictionary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> DecodeHintDictionary {
        DecodeHintDictionary(DecodeHints::default())
    }

    #[wasm_bindgen]
    pub fn get_hint(&self, hint: DecodeHintTypes) -> String {
        match hint {
            DecodeHintTypes::Other => self
                .0
                .Other
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            DecodeHintTypes::PureBarcode => self
                .0
                .PureBarcode
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::PossibleFormats => self
                .0
                .PossibleFormats
                .as_ref()
                .map(|v| {
                    v.iter()
                        .fold("".to_string(), |acc, v| acc + v.to_string().as_str())
                })
                .unwrap_or_default(),
            DecodeHintTypes::TryHarder => self
                .0
                .TryHarder
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::CharacterSet => self
                .0
                .CharacterSet
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            DecodeHintTypes::AllowedLengths => self
                .0
                .AllowedLengths
                .as_ref()
                .map(|v| {
                    v.iter()
                        .fold("".to_string(), |acc, v| acc + v.to_string().as_str())
                })
                .unwrap_or_default(),
            DecodeHintTypes::AssumeCode39CheckDigit => self
                .0
                .AssumeCode39CheckDigit
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::AssumeGs1 => self
                .0
                .AssumeGs1
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::ReturnCodabarStartEnd => self
                .0
                .ReturnCodabarStartEnd
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::NeedResultPointCallback => self
                .0
                .NeedResultPointCallback
                .as_ref()
                .map(|_| String::from("UNSUPORTED NeedResultPointCallback"))
                .unwrap_or_default(),
            DecodeHintTypes::AllowedEanExtensions => self
                .0
                .AllowedEanExtensions
                .as_ref()
                .map(|v| {
                    v.iter()
                        .fold("".to_string(), |acc, v| acc + v.to_string().as_str())
                })
                .unwrap_or_default(),
            DecodeHintTypes::AlsoInverted => self
                .0
                .AlsoInverted
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            DecodeHintTypes::TelepenAsNumeric => self
                .0
                .TelepenAsNumeric
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        }
        // if let Some(val) = self.0.get(&hint.into()) {
        //     match val {
        //         rxing::DecodeHintValue::Other(val) | rxing::DecodeHintValue::CharacterSet(val) => {
        //             val.to_owned()
        //         }
        //         rxing::DecodeHintValue::PureBarcode(val)
        //         | rxing::DecodeHintValue::AssumeCode39CheckDigit(val)
        //         | rxing::DecodeHintValue::AssumeGs1(val)
        //         | rxing::DecodeHintValue::ReturnCodabarStartEnd(val)
        //         | rxing::DecodeHintValue::TryHarder(val)
        //         | rxing::DecodeHintValue::TelepenAsNumeric(val)
        //         | rxing::DecodeHintValue::AlsoInverted(val) => val.to_string(),

        //         rxing::DecodeHintValue::PossibleFormats(val) => val
        //             .iter()
        //             .fold("".to_string(), |acc, v| acc + v.to_string().as_str()),
        //         rxing::DecodeHintValue::AllowedLengths(val)
        //         | rxing::DecodeHintValue::AllowedEanExtensions(val) => val
        //             .iter()
        //             .fold("".to_string(), |acc, v| acc + v.to_string().as_str()),
        //         rxing::DecodeHintValue::NeedResultPointCallback(_) => String::from("UNSUPORTED"),
        //     }
        // } else {
        //     String::from("")
        // }
    }

    #[wasm_bindgen]
    pub fn set_hint(&mut self, hint: DecodeHintTypes, value: String) -> bool {
        if value.is_empty() {
            return false;
        }

        match hint {
            DecodeHintTypes::Other => {
                self.0.Other = Some(value);
            }
            DecodeHintTypes::PureBarcode => {
                let Ok(pure_barcode) = value.parse() else {
                    return false;
                };
                self.0.PureBarcode = Some(pure_barcode);
            }
            DecodeHintTypes::PossibleFormats => {
                let formats = HashSet::from_iter(value.split(',').map(rxing::BarcodeFormat::from));

                if formats.is_empty() {
                    return false;
                }

                self.0.PossibleFormats = Some(formats);
            }
            DecodeHintTypes::TryHarder => {
                let Ok(try_harder) = value.parse() else {
                    return false;
                };
                self.0.TryHarder = Some(try_harder);
            }
            DecodeHintTypes::CharacterSet => {
                self.0.CharacterSet = Some(value);
            }
            DecodeHintTypes::AllowedLengths => {
                let lengths = value.split(',').flat_map(|v| v.parse()).collect();

                self.0.AllowedLengths = Some(lengths);
            }
            DecodeHintTypes::AssumeCode39CheckDigit => {
                let Ok(assume_code_39_check_digit) = value.parse() else {
                    return false;
                };
                self.0.AssumeCode39CheckDigit = Some(assume_code_39_check_digit);
            }
            DecodeHintTypes::AssumeGs1 => {
                let Ok(assume_gs1) = value.parse() else {
                    return false;
                };
                self.0.AssumeGs1 = Some(assume_gs1);
            }
            DecodeHintTypes::ReturnCodabarStartEnd => {
                let Ok(return_codebar_start_end) = value.parse() else {
                    return false;
                };
                self.0.ReturnCodabarStartEnd = Some(return_codebar_start_end);
            }
            DecodeHintTypes::NeedResultPointCallback => {
                return false;
            }
            DecodeHintTypes::AllowedEanExtensions => {
                let extensions = value.split(',').flat_map(|v| v.parse()).collect();

                self.0.AllowedEanExtensions = Some(extensions);
            }
            DecodeHintTypes::AlsoInverted => {
                let Ok(also_inverted) = value.parse() else {
                    return false;
                };
                self.0.AlsoInverted = Some(also_inverted);
            }
            DecodeHintTypes::TelepenAsNumeric => {
                let Ok(telepen_as_numeric) = value.parse() else {
                    return false;
                };
                self.0.TelepenAsNumeric = Some(telepen_as_numeric);
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn remove_hint(&mut self, hint: DecodeHintTypes) -> bool {
        match hint {
            DecodeHintTypes::Other => self.0.Other = None,
            DecodeHintTypes::PureBarcode => self.0.PureBarcode = None,
            DecodeHintTypes::PossibleFormats => self.0.PossibleFormats = None,
            DecodeHintTypes::TryHarder => self.0.TryHarder = None,
            DecodeHintTypes::CharacterSet => self.0.CharacterSet = None,
            DecodeHintTypes::AllowedLengths => self.0.AllowedLengths = None,
            DecodeHintTypes::AssumeCode39CheckDigit => self.0.AssumeCode39CheckDigit = None,
            DecodeHintTypes::AssumeGs1 => self.0.AssumeGs1 = None,
            DecodeHintTypes::ReturnCodabarStartEnd => self.0.ReturnCodabarStartEnd = None,
            DecodeHintTypes::NeedResultPointCallback => self.0.NeedResultPointCallback = None,
            DecodeHintTypes::AllowedEanExtensions => self.0.AllowedEanExtensions = None,
            DecodeHintTypes::AlsoInverted => self.0.AlsoInverted = None,
            DecodeHintTypes::TelepenAsNumeric => self.0.TelepenAsNumeric = None,
        }
        true
    }

    // #[wasm_bindgen]
    // pub fn setResultPointCallback(&mut self, callback: &js_sys::Function){
    //     self.0.insert(rxing::DecodeHintType::NEED_RESULT_POINT_CALLBACK, rxing::DecodeHintValue::NeedResultPointCallback(
    //         Rc::new(|point: &dyn rxing::ResultPoint|  {
    //             let this = JsValue::null();
    //             let js_point = wasm_bindgen::JsValue::from( js_sys::Array::from_iter([wasm_bindgen::JsValue::from(point.getX()), wasm_bindgen::JsValue::from(point.getY())].iter()) );
    //             callback.call1(&this, &js_point);
    //         })
    //     ));
    // }

    // #[wasm_bindgen]
    // pub fn clearResultPointCallback(&mut self) -> bool {
    //     self.0.remove(&rxing::DecodeHintType::NEED_RESULT_POINT_CALLBACK).is_some()
    // }
}

impl DecodeHintDictionary {
    pub fn get_dictionary(&self) -> &DecodeHints {
        &self.0
    }
    pub fn get_dictionary_mut(&mut self) -> &mut DecodeHints {
        &mut self.0
    }
}
