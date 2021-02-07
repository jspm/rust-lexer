
let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}
/**
* @param {string} input
* @returns {SourceAnalysis}
*/
export function parse(input) {
    var ptr0 = passStringToWasm0(input, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    var ret = wasm.parse(ptr0, len0);
    return SourceAnalysis.__wrap(ret);
}

/**
*/
export class DynamicImport {

    static __wrap(ptr) {
        const obj = Object.create(DynamicImport.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_dynamicimport_free(ptr);
    }
    /**
    * @returns {Range}
    */
    moduleSpecifierExpressionRange() {
        var ret = wasm.dynamicimport_moduleSpecifierExpressionRange(this.ptr);
        return Range.__wrap(ret);
    }
    /**
    * @returns {Range}
    */
    importExpressionRange() {
        var ret = wasm.dynamicimport_importExpressionRange(this.ptr);
        return Range.__wrap(ret);
    }
}
/**
*/
export class Export {

    static __wrap(ptr) {
        const obj = Object.create(Export.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_export_free(ptr);
    }
    /**
    * @returns {Range}
    */
    exportSpecifierRange() {
        var ret = wasm.export_exportSpecifierRange(this.ptr);
        return Range.__wrap(ret);
    }
}
/**
*/
export class ImportMeta {

    static __wrap(ptr) {
        const obj = Object.create(ImportMeta.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_importmeta_free(ptr);
    }
    /**
    * @returns {Range}
    */
    expressionRange() {
        var ret = wasm.export_exportSpecifierRange(this.ptr);
        return Range.__wrap(ret);
    }
}
/**
*/
export class Range {

    static __wrap(ptr) {
        const obj = Object.create(Range.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_range_free(ptr);
    }
    /**
    * @returns {number}
    */
    get start() {
        var ret = wasm.__wbg_get_range_start(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set start(arg0) {
        wasm.__wbg_set_range_start(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get end() {
        var ret = wasm.__wbg_get_range_end(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set end(arg0) {
        wasm.__wbg_set_range_end(this.ptr, arg0);
    }
}
/**
*/
export class SourceAnalysis {

    static __wrap(ptr) {
        const obj = Object.create(SourceAnalysis.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_sourceanalysis_free(ptr);
    }
    /**
    * @returns {Array<any>}
    */
    get imports() {
        var ret = wasm.sourceanalysis_imports(this.ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    get exports() {
        var ret = wasm.sourceanalysis_exports(this.ptr);
        return takeObject(ret);
    }
}
/**
*/
export class StaticImport {

    static __wrap(ptr) {
        const obj = Object.create(StaticImport.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_staticimport_free(ptr);
    }
    /**
    * @returns {Range}
    */
    moduleSpecifierRange() {
        var ret = wasm.dynamicimport_moduleSpecifierExpressionRange(this.ptr);
        return Range.__wrap(ret);
    }
    /**
    * @returns {Range}
    */
    statementRange() {
        var ret = wasm.staticimport_statementRange(this.ptr);
        return Range.__wrap(ret);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_export_new = function(arg0) {
        var ret = Export.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_staticimport_new = function(arg0) {
        var ret = StaticImport.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_dynamicimport_new = function(arg0) {
        var ret = DynamicImport.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_importmeta_new = function(arg0) {
        var ret = ImportMeta.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_1abc33d4f9ba3e80 = function() {
        var ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_44968dcdf4cfbb43 = function(arg0, arg1) {
        var ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

