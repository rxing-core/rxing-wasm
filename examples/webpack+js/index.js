import { convert_js_image_to_luma, decode_barcode_with_hints, DecodeHintDictionary, DecodeHintTypes, BarcodeFormat } from "rxing-wasm";

const text_hints = ["Other", "PossibleFormats", "CharacterSet", "AllowedLengths", "AllowedEanExtensions"];
const bool_hints = ["PureBarcode", "TryHarder", "AssumeCode39CheckDigit", "ReturnCodabarStartEnd", "AssumeGs1", "AlsoInverted", "TelepenAsNumeric"]

const scan_btn = document.getElementById('scan_btn');
const input = document.getElementById('image_file_input');
const output = document.getElementById("output");

input.addEventListener('change', handleFiles);
scan_btn.addEventListener('click', onClickScan);

function handleFiles(e) {
    scan_btn.disabled = true;
    output.hidden = true;
    const canvas = document.getElementById('cvs');
    const ctx = canvas.getContext('2d');
    const img = new Image;
    img.src = URL.createObjectURL(e.target.files[0]);
    img.onload = function () {
        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0, img.width, img.height);
        scan_btn.disabled = false;
    }
}

function onClickScan() {
    const canvas = document.getElementById('cvs');
    const context = canvas.getContext('2d');
    const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    const luma_data = convert_js_image_to_luma(imageData.data);
    const hints = getHints();
    let result;
    try {
        result = decode_barcode_with_hints(luma_data, canvas.width, canvas.height, hints);
    } catch (e) {
        alert("Issue decoding: " + e);
    }
    write_results(result.format(), result.text(), result.raw_bytes(), result.result_points(), result.get_meta_data());
}

function write_results(format, text, raw_bytes, _points, metadata) {
    // const points_formatted = [];
    // const chunkSize = 2;
    // console.log(JSON.stringify(points));
    // for (let i = 0; i < points.length; i += chunkSize) {
    //     const chunk = points.slice(i, i + chunkSize);
    //     points_formatted.push([chunk[0], chunk[1]]);
    // }

    let metadata_string = "";
    metadata.forEach((k,v) => {metadata_string += `${k}: ${v}\n`});
    // for (const md_k of metadata.keys()){
    //     // console.log(`${md_k}: ${metadata.get(md_k)}\n`);
    //     metadata += `${md_k}: ${metadata.get(md_k)}\n`;
    // }

    document.getElementById("text_result_td").innerText = text;

    document.getElementById("format_result_td").innerText = BarcodeFormat[format];

    // document.getElementById("points_result_td").innerText = points_formatted.reduce((acc, point) => { acc + "(" + point[0] + "," + point[1], ") " }, "");

    document.getElementById("raw_bytes_result_td").innerText = Object.keys(raw_bytes).map((k) => raw_bytes[k]).join(', '); //.toString().split(",");//.reduce((acc,v)=>{acc + "-" + v}, "");

    document.getElementById("medata_data_result_td").innerText = metadata_string;

    output.hidden = false;
}

function get_text_hint(id) {
    const input = document.getElementById(id);
    return input.value;
}

function get_bool_hint(id) {
    const input = document.getElementById(id);
    return input.checked.toString();
}

function getHints() {
    const hint_dictionary = new DecodeHintDictionary();
    for (const hint of text_hints) {
        hint_dictionary.set_hint(DecodeHintTypes[hint], get_text_hint(hint));
    }
    for (const hint of bool_hints) {
        hint_dictionary.set_hint(DecodeHintTypes[hint], get_bool_hint(hint));
    }
    if (get_bool_hint("PureBarcode") == "false") {
        hint_dictionary.remove_hint(DecodeHintTypes["PureBarcode"]);
    }
    return hint_dictionary;
}