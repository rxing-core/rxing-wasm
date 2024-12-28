use rxing::{datamatrix::encoder::SymbolShapeHint, Dimension, EncodeHints};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum EncodeHintTypes {
    /**
     * Specifies what degree of error correction to use, for example in QR Codes.
     * Type depends on the encoder. For example for QR codes it's type
     * {@link com.google.zxing.qrcode.decoder.ErrorCorrectionLevel ErrorCorrectionLevel}.
     * For Aztec it is of type {@link Integer}, representing the minimal percentage of error correction words.
     * For PDF417 it is of type {@link Integer}, valid values being 0 to 8.
     * In all cases, it can also be a {@link String} representation of the desired value as well.
     * Note: an Aztec symbol should have a minimum of 25% EC words.
     */
    ErrorCorrection,

    /**
     * Specifies what character encoding to use where applicable (type {@link String})
     */
    CharacterSet,

    /**
     * Specifies the matrix shape for Data Matrix (type {@link com.google.zxing.datamatrix.encoder.SymbolShapeHint})
     */
    DataMatrixShape,

    /**
     * Specifies whether to use compact mode for Data Matrix (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * The compact encoding mode also supports the encoding of characters that are not in the ISO-8859-1
     * character set via ECIs.
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     * Compact encoding also provides GS1-FNC1 support when {@link #GS1_FORMAT} is selected. In this case
     * group-separator character (ASCII 29 decimal) can be used to encode the positions of FNC1 codewords
     * for the purpose of delimiting AIs.
     * This option and {@link #FORCE_C40} are mutually exclusive.
     */
    DataMatrixCompact,

    /**
     * Specifies a minimum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated use width/height params in
     * {@link com.google.zxing.datamatrix.DataMatrixWriter#encode(String, BarcodeFormat, int, int)}
     */
    #[deprecated]
    MinSize,

    /**
     * Specifies a maximum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated without replacement
     */
    #[deprecated]
    MaxSize,

    /**
     * Specifies margin, in pixels, to use when generating the barcode. The meaning can vary
     * by format; for example it controls margin before and after the barcode horizontally for
     * most 1D formats. (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    MARGIN,

    /**
     * Specifies whether to use compact mode for PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     */
    Pdf417Compact,

    /**
     * Specifies what compaction mode to use for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Compaction Compaction} or {@link String} value of one of its
     * enum values).
     */
    Pdf417Compaction,

    /**
     * Specifies the minimum and maximum number of rows and columns for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Dimensions Dimensions}).
     */
    Pdf417Dimensions,

    /**
     * Specifies whether to automatically insert ECIs when encoding PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     */
    Pdf417AutoEci,

    /**
     * Specifies the required number of layers for an Aztec code.
     * A negative number (-1, -2, -3, -4) specifies a compact Aztec code.
     * 0 indicates to use the minimum number of layers (the default).
     * A positive number (1, 2, .. 32) specifies a normal (non-compact) Aztec code.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    AztecLayers,

    /**
     * Specifies the exact version of QR code to be encoded.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QrVersion,

    /**
     * Specifies the QR code mask pattern to be used. Allowed values are
     * 0..QRCode.NUM_MASK_PATTERNS-1. By default the code will automatically select
     * the optimal mask pattern.
     * * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QrMaskPattern,

    /**
     * Specifies whether to use compact mode for QR code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * Please note that when compaction is performed, the most compact character encoding is chosen
     * for characters in the input that are not in the ISO-8859-1 character set. Based on experience,
     * some scanners do not support encodings like cp-1256 (Arabic). In such cases the encoding can
     * be forced to UTF-8 by means of the {@link #CHARACTER_SET} encoding hint.
     */
    QrCompact,

    /**
     * Specifies whether the data should be encoded to the GS1 standard (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     */
    Gs1Format,

    /**
     * Forces which encoding will be used. Currently only used for Code-128 code sets (Type {@link String}).
     * Valid values are "A", "B", "C".
     * This option and {@link #CODE128_COMPACT} are mutually exclusive.
     */
    ForceCodeSet,

    /**
     * Forces C40 encoding for data-matrix (type {@link Boolean}, or "true" or "false") {@link String } value). This
     * option and {@link #DATA_MATRIX_COMPACT} are mutually exclusive.
     */
    ForceC40,

    /**
     * Specifies whether to use compact mode for Code-128 code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * This can yield slightly smaller bar codes. This option and {@link #FORCE_CODE_SET} are mutually
     * exclusive.
     */
    Code128Compact,

    /*
     * Will translate the numeric values received by the Telepen writer into the Telepen Alphanumeric form.
     */
    TelepenAsNumeric,
}

impl From<EncodeHintTypes> for rxing::EncodeHintType {
    fn from(value: EncodeHintTypes) -> Self {
        match value {
            EncodeHintTypes::ErrorCorrection => rxing::EncodeHintType::ERROR_CORRECTION,
            EncodeHintTypes::CharacterSet => rxing::EncodeHintType::CHARACTER_SET,
            EncodeHintTypes::DataMatrixShape => rxing::EncodeHintType::DATA_MATRIX_SHAPE,
            EncodeHintTypes::DataMatrixCompact => rxing::EncodeHintType::DATA_MATRIX_COMPACT,
            EncodeHintTypes::MinSize => rxing::EncodeHintType::MIN_SIZE,
            EncodeHintTypes::MaxSize => rxing::EncodeHintType::MAX_SIZE,
            EncodeHintTypes::MARGIN => rxing::EncodeHintType::MARGIN,
            EncodeHintTypes::Pdf417Compact => rxing::EncodeHintType::PDF417_COMPACT,
            EncodeHintTypes::Pdf417Compaction => rxing::EncodeHintType::PDF417_COMPACTION,
            EncodeHintTypes::Pdf417Dimensions => rxing::EncodeHintType::PDF417_DIMENSIONS,
            EncodeHintTypes::Pdf417AutoEci => rxing::EncodeHintType::PDF417_AUTO_ECI,
            EncodeHintTypes::AztecLayers => rxing::EncodeHintType::AZTEC_LAYERS,
            EncodeHintTypes::QrVersion => rxing::EncodeHintType::QR_VERSION,
            EncodeHintTypes::QrMaskPattern => rxing::EncodeHintType::QR_MASK_PATTERN,
            EncodeHintTypes::QrCompact => rxing::EncodeHintType::QR_COMPACT,
            EncodeHintTypes::Gs1Format => rxing::EncodeHintType::GS1_FORMAT,
            EncodeHintTypes::ForceCodeSet => rxing::EncodeHintType::FORCE_CODE_SET,
            EncodeHintTypes::ForceC40 => rxing::EncodeHintType::FORCE_C40,
            EncodeHintTypes::Code128Compact => rxing::EncodeHintType::CODE128_COMPACT,
            EncodeHintTypes::TelepenAsNumeric => rxing::EncodeHintType::TELEPEN_AS_NUMERIC,
        }
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct EncodeHintDictionary(EncodeHints);

#[wasm_bindgen]
impl EncodeHintDictionary {
    #[wasm_bindgen(constructor)]
    pub fn new() -> EncodeHintDictionary {
        EncodeHintDictionary(EncodeHints::default())
    }

    #[wasm_bindgen]
    pub fn get_hint(&self, hint: EncodeHintTypes) -> String {
        match hint {
            EncodeHintTypes::ErrorCorrection => self
                .0
                .ErrorCorrection
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::CharacterSet => self
                .0
                .CharacterSet
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::DataMatrixShape => self
                .0
                .DataMatrixShape
                .as_ref()
                .map(|v| match v {
                    SymbolShapeHint::FORCE_NONE => "ForceNone".to_owned(),
                    SymbolShapeHint::FORCE_RECTANGLE => "ForceRectangle".to_owned(),
                    SymbolShapeHint::FORCE_SQUARE => "ForceSquare".to_owned(),
                })
                .unwrap_or_default(),
            EncodeHintTypes::DataMatrixCompact => self
                .0
                .DataMatrixCompact
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::MinSize => self
                .0
                .MinSize
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::MaxSize => self
                .0
                .MaxSize
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::MARGIN => self
                .0
                .Margin
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::Pdf417Compact => self
                .0
                .Pdf417Compact
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::Pdf417Compaction => self
                .0
                .Pdf417Compaction
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::Pdf417Dimensions => self
                .0
                .Pdf417Dimensions
                .as_ref()
                .map(|v| {
                    format!(
                        "{}/{}||{}/{}",
                        v.getMinCols(),
                        v.getMaxCols(),
                        v.getMinRows(),
                        v.getMaxRows()
                    )
                })
                .unwrap_or_default(),
            EncodeHintTypes::Pdf417AutoEci => self
                .0
                .Pdf417AutoEci
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::AztecLayers => self
                .0
                .AztecLayers
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::QrVersion => self
                .0
                .QrVersion
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::QrMaskPattern => self
                .0
                .QrMaskPattern
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::QrCompact => self
                .0
                .QrCompact
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::Gs1Format => self
                .0
                .Gs1Format
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::ForceCodeSet => self
                .0
                .ForceCodeSet
                .as_ref()
                .map(|v| v.to_owned())
                .unwrap_or_default(),
            EncodeHintTypes::ForceC40 => self
                .0
                .ForceC40
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::Code128Compact => self
                .0
                .Code128Compact
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
            EncodeHintTypes::TelepenAsNumeric => self
                .0
                .TelepenAsNumeric
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or_default(),
        }
    }
    // if let Some(val) = self.0.get(&hint.into()) {
    //     match val {
    //         rxing::EncodeHintValue::ErrorCorrection(val)
    //         | rxing::EncodeHintValue::CharacterSet(val)
    //         | rxing::EncodeHintValue::Margin(val)
    //         | rxing::EncodeHintValue::Pdf417Compact(val)
    //         | rxing::EncodeHintValue::Pdf417Compaction(val)
    //         | rxing::EncodeHintValue::Pdf417AutoEci(val)
    //         | rxing::EncodeHintValue::QrVersion(val)
    //         | rxing::EncodeHintValue::QrMaskPattern(val)
    //         | rxing::EncodeHintValue::QrCompact(val)
    //         | rxing::EncodeHintValue::ForceCodeSet(val) => val.to_owned(),

    //         rxing::EncodeHintValue::DataMatrixCompact(val)
    //         | rxing::EncodeHintValue::Gs1Format(val)
    //         | rxing::EncodeHintValue::ForceC40(val)
    //         | rxing::EncodeHintValue::Code128Compact(val)
    //         | rxing::EncodeHintValue::TelepenAsNumeric(val) => val.to_string(),

    //         rxing::EncodeHintValue::AztecLayers(val) => val.to_string(),

    //         rxing::EncodeHintValue::DataMatrixShape(val) => match val {
    //             SymbolShapeHint::FORCE_NONE => "ForceNone".to_owned(),
    //             SymbolShapeHint::FORCE_RECTANGLE => "ForceRectangle".to_owned(),
    //             SymbolShapeHint::FORCE_SQUARE => "ForceSquare".to_owned(),
    //         },

    //         rxing::EncodeHintValue::MinSize(val) | rxing::EncodeHintValue::MaxSize(val) => {
    //             val.to_string()
    //         }
    //         rxing::EncodeHintValue::Pdf417Dimensions(val) => format!(
    //             "{}/{}||{}/{}",
    //             val.getMinCols(),
    //             val.getMaxCols(),
    //             val.getMinRows(),
    //             val.getMaxRows()
    //         ),
    //     }
    // } else {
    //     String::from("")
    // }

    #[wasm_bindgen]
    pub fn set_hint(&mut self, hint: EncodeHintTypes, value: String) -> bool {
        if value.is_empty() {
            return false;
        }

        match hint {
            EncodeHintTypes::ErrorCorrection => {
                self.0.ErrorCorrection = Some(value);
            }
            EncodeHintTypes::CharacterSet => {
                self.0.CharacterSet = Some(value);
            }
            EncodeHintTypes::DataMatrixShape => {
                let shape_hint = match value.as_str() {
                    "ForceNone" => SymbolShapeHint::FORCE_NONE,
                    "ForceRectangle" => SymbolShapeHint::FORCE_RECTANGLE,
                    "ForceSquare" => SymbolShapeHint::FORCE_SQUARE,
                    _ => {
                        return false;
                    }
                };
                self.0.DataMatrixShape = Some(shape_hint);
            }
            EncodeHintTypes::DataMatrixCompact => {
                let Ok(parsed_bool) = value.parse() else {
                    return false;
                };
                self.0.DataMatrixCompact = Some(parsed_bool);
            }
            EncodeHintTypes::MinSize => {
                let Some(dim) = parse_dimension(&value) else {
                    return false;
                };
                self.0.MinSize = Some(dim);
            }
            EncodeHintTypes::MaxSize => {
                let Some(dim) = parse_dimension(&value) else {
                    return false;
                };
                self.0.MaxSize = Some(dim);
            }
            EncodeHintTypes::MARGIN => {
                self.0.Margin = Some(value);
            }
            EncodeHintTypes::Pdf417Compact => {
                self.0.Pdf417Compact = Some(value);
            }
            EncodeHintTypes::Pdf417Compaction => {
                self.0.Pdf417Compaction = Some(value);
            }
            EncodeHintTypes::Pdf417Dimensions => {
                let Some(dim) = parse_dimensions(&value) else {
                    return false;
                };
                self.0.Pdf417Dimensions = Some(dim);
            }
            EncodeHintTypes::Pdf417AutoEci => {
                self.0.Pdf417AutoEci = Some(value);
            }
            EncodeHintTypes::AztecLayers => {
                let Ok(parsed_i32) = value.parse() else {
                    return false;
                };
                self.0.AztecLayers = Some(parsed_i32);
            }
            EncodeHintTypes::QrVersion => {
                self.0.QrVersion = Some(value);
            }
            EncodeHintTypes::QrMaskPattern => {
                self.0.QrMaskPattern = Some(value);
            }
            EncodeHintTypes::QrCompact => {
                self.0.QrCompact = Some(value);
            }
            EncodeHintTypes::Gs1Format => {
                let Ok(parsed_bool) = value.parse() else {
                    return false;
                };
                self.0.Gs1Format = Some(parsed_bool);
            }
            EncodeHintTypes::ForceCodeSet => {
                self.0.ForceCodeSet = Some(value);
            }
            EncodeHintTypes::ForceC40 => {
                let Ok(parsed_bool) = value.parse() else {
                    return false;
                };
                self.0.ForceC40 = Some(parsed_bool);
            }
            EncodeHintTypes::Code128Compact => {
                let Ok(code128_compact) = value.parse() else {
                    return false;
                };
                self.0.Code128Compact = Some(code128_compact);
            }
            EncodeHintTypes::TelepenAsNumeric => {
                let Ok(telepen_numeric) = value.parse() else {
                    return false;
                };
                self.0.TelepenAsNumeric = Some(telepen_numeric);
            }
        }
        true
    }

    #[wasm_bindgen]
    pub fn remove_hint(&mut self, hint: EncodeHintTypes) -> bool {
        match hint {
            EncodeHintTypes::ErrorCorrection => self.0.ErrorCorrection = None,
            EncodeHintTypes::CharacterSet => self.0.CharacterSet = None,
            EncodeHintTypes::DataMatrixShape => self.0.DataMatrixShape = None,
            EncodeHintTypes::DataMatrixCompact => self.0.DataMatrixCompact = None,
            EncodeHintTypes::MinSize => self.0.MinSize = None,
            EncodeHintTypes::MaxSize => self.0.MaxSize = None,
            EncodeHintTypes::MARGIN => self.0.Margin = None,
            EncodeHintTypes::Pdf417Compact => self.0.Pdf417Compact = None,
            EncodeHintTypes::Pdf417Compaction => self.0.Pdf417Compaction = None,
            EncodeHintTypes::Pdf417Dimensions => self.0.Pdf417Dimensions = None,
            EncodeHintTypes::Pdf417AutoEci => self.0.Pdf417AutoEci = None,
            EncodeHintTypes::AztecLayers => self.0.AztecLayers = None,
            EncodeHintTypes::QrVersion => self.0.QrVersion = None,
            EncodeHintTypes::QrMaskPattern => self.0.QrMaskPattern = None,
            EncodeHintTypes::QrCompact => self.0.QrCompact = None,
            EncodeHintTypes::Gs1Format => self.0.Gs1Format = None,
            EncodeHintTypes::ForceCodeSet => self.0.ForceCodeSet = None,
            EncodeHintTypes::ForceC40 => self.0.ForceC40 = None,
            EncodeHintTypes::Code128Compact => self.0.Code128Compact = None,
            EncodeHintTypes::TelepenAsNumeric => self.0.TelepenAsNumeric = None,
        }
        true
    }
}

impl EncodeHintDictionary {
    pub fn get_dictionary(&self) -> &EncodeHints {
        &self.0
    }
    pub fn get_dictionary_mut(&mut self) -> &mut EncodeHints {
        &mut self.0
    }
}

fn parse_dimension(dim: &str) -> Option<rxing::Dimension> {
    if dim.is_empty() {
        return None;
    }
    if !dim.contains("x") {
        return None;
    }

    let Some(x_pos) = dim.find("x") else {
        return None;
    };

    let p1 = &dim[..x_pos];
    let p2 = &dim[x_pos + 1..];

    if p1.is_empty() || p2.is_empty() {
        return None;
    }

    let Ok(p1_parse) = p1.parse() else {
        return None;
    };

    let Ok(p2_parse) = p2.parse() else {
        return None;
    };

    Some(Dimension::new(p1_parse, p2_parse))
}

fn parse_dimensions(dim: &str) -> Option<rxing::pdf417::encoder::Dimensions> {
    // "MINCOL:{}/MAXCOL:{}||MINROW:{}/MAXROW:{}"
    if dim.is_empty() {
        return None;
    }
    if !dim.contains("||") {
        return None;
    }
    let Some(split) = dim.find("||") else {
        return None;
    };
    let Some((min_cols, max_cols)) = parse_dimensions_sub_part(&dim[..split]) else {
        return None;
    };
    let Some((min_rows, max_rows)) = parse_dimensions_sub_part(&dim[split + 1..]) else {
        return None;
    };

    Some(rxing::pdf417::encoder::Dimensions::new(
        min_cols, max_cols, min_rows, max_rows,
    ))
}

fn parse_dimensions_sub_part(part: &str) -> Option<(usize, usize)> {
    if part.is_empty() {
        return None;
    }
    if !part.contains("/") {
        return None;
    }
    let Some(split) = part.find("/") else {
        return None;
    };
    let p1 = &part[..split];
    let p2 = &part[split + 1..];

    if p1.is_empty() || p2.is_empty() {
        return None;
    }

    let Ok(p1_parse) = p1.parse() else {
        return None;
    };

    let Ok(p2_parse) = p2.parse() else {
        return None;
    };
    Some((p1_parse, p2_parse))
}
