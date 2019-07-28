function fetch_inline_binary_base64(input) {
    return new Promise(function (resolve, reject) {
        let binary_string = window.atob(input);
        let bytes = new Uint8Array(binary_string.length);
        for (let i = 0; i < binary_string.length; i++) {
            bytes[i] = binary_string.charCodeAt(i);
        }
        resolve(bytes.buffer);
    });
}

function fetch_remote_binary(url) {
    let req = new XMLHttpRequest();

    return new Promise(function (resolve, reject) {
        req.onreadystatechange = function () {
            if (req.readyState !== 4) return;

            if (req.status >= 200 && req.status < 300) {
                resolve(req.response);
            } else {
                reject({
                    status: req.status,
                    statusText: req.statusText
                });
            }
        };

        req.open('GET', url, true);
        req.responseType = "arraybuffer";
        req.send();
    });
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

function transform_binary_litcontainer(buffer) {
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

function prepare_litcontainer_data(container) {
    if (container.rows === 1 || container.cols === 1) {
        return Array.from(container.data)
    } else if (container.col_stride === 1) {
        let ret = [];
        for (let i = 0; i < container.rows * container.cols; i += container.cols) {
            ret.push(Array.from(container.data.slice(i, i + container.cols)));
        }
        return ret;
    } else if (container.row_stride === 1) {
        let ret = [];
        for (let i = 0; i < container.rows * container.cols; i += container.rows) {
            ret.push(Array.from(container.data.slice(i, i + container.rows)));
        }
        return ret;
    } else {
        return Array.from(container.data)
    }
}