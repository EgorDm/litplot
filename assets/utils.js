function load_binary_base64(data, callback) {
    let binary_string = window.atob(data);
    let bytes = new Uint8Array(binary_string.length);
    for (let i = 0; i < binary_string.length; i++) {
        bytes[i] = binary_string.charCodeAt(i);
    }
    callback(bytes.buffer);
}

function load_binary_resource(url, callback) {
    let req = new XMLHttpRequest();
    req.open('GET', url, true);
    req.responseType = "arraybuffer";
    req.onload = function (event) {
        callback(req.response);
    };
    req.send(null);
}

function element_type_to_format(element_type) {
    let ret = {
        array_type: null,
        is_complex: (element_type & 1) === 1
    };

    element_type = element_type & ~1;
    switch (element_type) {
        case 2:
            ret.array_type = Float32Array;
            break;
        case 4:
            ret.array_type = Float64Array;
            break;
        case 8:
            ret.array_type = Uint8Array;
            break;
        case 16:
            ret.array_type = Int16Array;
            break;
        case 32:
            ret.array_type = Int32Array;
            break;
        case 64:
            ret.array_type = BigInt64Array;
            break;
    }

    return ret;
}

function parse_lit_container(buffer) {
    let dataView = new DataView(buffer);
    let tmp = 0, cursor = 0;
    let progress = (i) => {
        tmp = cursor;
        cursor += i;
        return tmp;
    };

    let header = {
        element_type: dataView.getUint8(progress(1)),
        element_size: Number(dataView.getBigUint64(progress(8), true)),
        rows: Number(dataView.getBigUint64(progress(8), true)),
        cols: Number(dataView.getBigUint64(progress(8), true)),
        row_stride: Number(dataView.getBigUint64(progress(8), true)),
        col_stride: Number(dataView.getBigUint64(progress(8), true)),
    };
    progress(8); // Skip the data size

    let format = element_type_to_format(header.element_type);
    let element_count = header.rows * header.cols;
    let data_element_count = format.is_complex ? element_count * 2 : element_count;

    let data = new format.array_type(buffer.slice(cursor, cursor + data_element_count * header.element_size));

    return {...header, data: data};
}