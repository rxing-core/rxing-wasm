import { convert_js_image_to_luma, decode_barcode_with_hints, decode_multi, DecodeHintDictionary, DecodeHintTypes, BarcodeFormat } from "rxing-wasm";

const text_hints = ["Other", "PossibleFormats", "CharacterSet", "AllowedLengths", "AllowedEanExtensions"];
const bool_hints = ["PureBarcode", "TryHarder", "AssumeCode39CheckDigit", "ReturnCodabarStartEnd", "AssumeGs1", "AlsoInverted", "TelepenAsNumeric"]

const scan_btn = document.getElementById('scan_btn');
const input = document.getElementById('image_file_input');
const output = document.getElementById("output_container");
const multi_btn = document.getElementById('scan_multi');

input.addEventListener('change', handleFiles);
scan_btn.addEventListener('click', onClickScan);
multi_btn.addEventListener('click', onClickMulti);

function handleFiles(e) {
    scan_btn.disabled = true;
    multi_btn.disabled = true;
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
        multi_btn.disabled = false;
    }
}

function onClickScan() {
    output.innerHTML = '';
    const canvas = document.getElementById('cvs');
    const context = canvas.getContext('2d');
    const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    const luma_data = convert_js_image_to_luma(imageData.data);
    const filter_image = document.getElementById("FilterInput").checked;
    const hints = getHints();
    let result;
    try {
        result = decode_barcode_with_hints(luma_data, canvas.width, canvas.height, hints, filter_image);
    } catch (e) {
        alert("Issue decoding: " + e);
    }
    write_results(result.format(), result.text(), result.raw_bytes(), result.result_points(), result.get_meta_data());

    output.hidden = false;
}

function onClickMulti() {
    output.innerHTML = '';
    const canvas = document.getElementById('cvs');
    const context = canvas.getContext('2d');
    const imageData = context.getImageData(0, 0, canvas.width, canvas.height);
    const luma_data = convert_js_image_to_luma(imageData.data);
    const filter_image = document.getElementById("FilterInput").checked;
    const hints = getHints();
    let result;
    try {
        result = decode_multi(luma_data, canvas.width, canvas.height, hints, filter_image);
    } catch (e) {
        alert("Issue decoding: " + e);
    }

    for (const res of result) {
        write_results(res.format(), res.text(), res.raw_bytes(), res.result_points(), res.get_meta_data());
    }

    output.hidden = false;
}

function write_results(format, text, raw_bytes, _points, metadata) {
    let metadata_string = "";
    metadata.forEach((k, v) => { metadata_string += `${k}: ${v}\n` });

    const new_element = document.createElement("rxing-result-display");

    add_template_slot("text_result", text, new_element);
    add_template_slot("format_result", BarcodeFormat[format], new_element);
    add_template_slot("raw_bytes_result", Object.keys(raw_bytes).map((k) => raw_bytes[k]).join(', '), new_element);
    add_template_slot("medata_data_result", metadata_string, new_element);

    output.appendChild(new_element);
}

function add_template_slot(slot_name, slot_data, template) {
    const new_slot_span = document.createElement("span");
    new_slot_span.setAttribute("slot", slot_name);
    new_slot_span.innerText = slot_data;
    template.appendChild(new_slot_span);
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

customElements.define(
    "rxing-result-display",
    class extends HTMLElement {
        constructor() {
            super();
            const template = document.getElementById(
                "result-element-template",
            ).content;
            const shadowRoot = this.attachShadow({ mode: "open" });
            shadowRoot.appendChild(template.cloneNode(true));
        }
    },
);