let wasm;

function addHeapObject(obj) {
  if (heap_next === heap.length)
    heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];

  heap[idx] = obj;
  return idx;
}

function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }
}

function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches && builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  return className;
}

function dropObject(idx) {
  if (idx < 132)
    return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function getArrayJsValueFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  const mem = getDataViewMemory0();
  const result = [];
  for (let i = ptr; i < ptr + 4 * len; i += 4) {
    result.push(takeObject(mem.getUint32(i, true)));
  }
  return result;
}

function getArrayU32FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
  if (cachedDataViewMemory0 === null ||
      cachedDataViewMemory0.buffer.detached === true ||
      (cachedDataViewMemory0.buffer.detached === undefined &&
       cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
  if (cachedUint32ArrayMemory0 === null ||
      cachedUint32ArrayMemory0.byteLength === 0) {
    cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
  }
  return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
  if (cachedUint8ArrayMemory0 === null ||
      cachedUint8ArrayMemory0.byteLength === 0) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    wasm.__wbindgen_export3(addHeapObject(e));
  }
}

let heap = new Array(128).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) { return x === undefined || x === null; }

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;

  const mem = getUint8ArrayMemory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7f)
      break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, (len = offset + arg.length * 3), 1) >>> 0;
    const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
    const ret = cachedTextEncoder.encodeInto(arg, view);

    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}

let cachedTextDecoder =
    new TextDecoder("utf-8", {ignoreBOM : true, fatal : true});
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder =
        new TextDecoder("utf-8", {ignoreBOM : true, fatal : true});
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(
      getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!("encodeInto" in cachedTextEncoder)) {
  cachedTextEncoder.encodeInto = function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read : arg.length,
      written : buf.length,
    };
  };
}

let WASM_VECTOR_LEN = 0;

const WasmConversionOptionsHandleFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) =>
                  wasm.__wbg_wasmconversionoptionshandle_free(ptr >>> 0, 1));

const WasmHtmlExtractionFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) => wasm.__wbg_wasmhtmlextraction_free(ptr >>> 0, 1));

const WasmInlineImageFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) => wasm.__wbg_wasminlineimage_free(ptr >>> 0, 1));

const WasmInlineImageConfigFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) => wasm.__wbg_wasminlineimageconfig_free(ptr >>> 0, 1));

const WasmInlineImageWarningFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) => wasm.__wbg_wasminlineimagewarning_free(ptr >>> 0, 1));

const WasmMetadataConfigFinalization =
    typeof FinalizationRegistry === "undefined"
        ? {register : () => {}, unregister : () => {}}
        : new FinalizationRegistry(
              (ptr) => wasm.__wbg_wasmmetadataconfig_free(ptr >>> 0, 1));

export class WasmConversionOptionsHandle {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(WasmConversionOptionsHandle.prototype);
    obj.__wbg_ptr = ptr;
    WasmConversionOptionsHandleFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmConversionOptionsHandleFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmconversionoptionshandle_free(ptr, 0);
  }
  /**
   * @param {any} options
   */
  constructor(options) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasmconversionoptionshandle_new(retptr, addHeapObject(options));
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
      if (r2) {
        throw takeObject(r1);
      }
      this.__wbg_ptr = r0 >>> 0;
      WasmConversionOptionsHandleFinalization.register(this, this.__wbg_ptr,
                                                       this);
      return this;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
}
if (Symbol.dispose)
  WasmConversionOptionsHandle.prototype[Symbol.dispose] =
      WasmConversionOptionsHandle.prototype.free;

/**
 * Result of HTML extraction with inline images
 */
export class WasmHtmlExtraction {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(WasmHtmlExtraction.prototype);
    obj.__wbg_ptr = ptr;
    WasmHtmlExtractionFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmHtmlExtractionFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmhtmlextraction_free(ptr, 0);
  }
  /**
   * @returns {WasmInlineImage[]}
   */
  get inlineImages() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasmhtmlextraction_inlineImages(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
      wasm.__wbindgen_export4(r0, r1 * 4, 4);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
   * @returns {string}
   */
  get markdown() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasmhtmlextraction_markdown(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @returns {WasmInlineImageWarning[]}
   */
  get warnings() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasmhtmlextraction_warnings(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
      wasm.__wbindgen_export4(r0, r1 * 4, 4);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
}
if (Symbol.dispose)
  WasmHtmlExtraction.prototype[Symbol.dispose] =
      WasmHtmlExtraction.prototype.free;

/**
 * Inline image data
 */
export class WasmInlineImage {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(WasmInlineImage.prototype);
    obj.__wbg_ptr = ptr;
    WasmInlineImageFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmInlineImageFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasminlineimage_free(ptr, 0);
  }
  /**
   * @returns {any}
   */
  get attributes() {
    const ret = wasm.wasminlineimage_attributes(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
   * @returns {Uint32Array | undefined}
   */
  get dimensions() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimage_dimensions(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      let v1;
      if (r0 !== 0) {
        v1 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
   * @returns {string | undefined}
   */
  get description() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimage_description(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      let v1;
      if (r0 !== 0) {
        v1 = getStringFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  /**
   * @returns {Uint8Array}
   */
  get data() {
    const ret = wasm.wasminlineimage_data(this.__wbg_ptr);
    return takeObject(ret);
  }
  /**
   * @returns {string}
   */
  get format() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimage_format(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @returns {string}
   */
  get source() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimage_source(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @returns {string | undefined}
   */
  get filename() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimage_filename(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      let v1;
      if (r0 !== 0) {
        v1 = getStringFromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 1, 1);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
}
if (Symbol.dispose)
  WasmInlineImage.prototype[Symbol.dispose] = WasmInlineImage.prototype.free;

/**
 * Inline image configuration
 */
export class WasmInlineImageConfig {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmInlineImageConfigFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasminlineimageconfig_free(ptr, 0);
  }
  /**
   * @param {boolean} capture
   */
  set captureSvg(capture) {
    wasm.wasminlineimageconfig_set_captureSvg(this.__wbg_ptr, capture);
  }
  /**
   * @param {string | null} [prefix]
   */
  set filenamePrefix(prefix) {
    var ptr0 = isLikeNone(prefix)
                   ? 0
                   : passStringToWasm0(prefix, wasm.__wbindgen_export,
                                       wasm.__wbindgen_export2);
    var len0 = WASM_VECTOR_LEN;
    wasm.wasminlineimageconfig_set_filenamePrefix(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @param {boolean} infer
   */
  set inferDimensions(infer) {
    wasm.wasminlineimageconfig_set_inferDimensions(this.__wbg_ptr, infer);
  }
  /**
   * @param {number | null} [max_decoded_size_bytes]
   */
  constructor(max_decoded_size_bytes) {
    const ret = wasm.wasminlineimageconfig_new(
        !isLikeNone(max_decoded_size_bytes),
        isLikeNone(max_decoded_size_bytes) ? 0 : max_decoded_size_bytes,
    );
    this.__wbg_ptr = ret >>> 0;
    WasmInlineImageConfigFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
}
if (Symbol.dispose)
  WasmInlineImageConfig.prototype[Symbol.dispose] =
      WasmInlineImageConfig.prototype.free;

/**
 * Warning about inline image processing
 */
export class WasmInlineImageWarning {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(WasmInlineImageWarning.prototype);
    obj.__wbg_ptr = ptr;
    WasmInlineImageWarningFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmInlineImageWarningFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasminlineimagewarning_free(ptr, 0);
  }
  /**
   * @returns {number}
   */
  get index() {
    const ret = wasm.wasminlineimagewarning_index(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @returns {string}
   */
  get message() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.wasminlineimagewarning_message(retptr, this.__wbg_ptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
  }
}
if (Symbol.dispose)
  WasmInlineImageWarning.prototype[Symbol.dispose] =
      WasmInlineImageWarning.prototype.free;

/**
 * Metadata extraction configuration
 */
export class WasmMetadataConfig {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmMetadataConfigFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmmetadataconfig_free(ptr, 0);
  }
  /**
   * @returns {boolean}
   */
  get extract_links() {
    const ret = wasm.wasmmetadataconfig_extract_links(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {boolean}
   */
  get extract_images() {
    const ret = wasm.wasmmetadataconfig_extract_images(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {boolean}
   */
  get extract_headers() {
    const ret = wasm.wasmmetadataconfig_extract_headers(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {boolean}
   */
  get extract_document() {
    const ret = wasm.wasmmetadataconfig_extract_document(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @param {boolean} value
   */
  set extract_links(value) {
    wasm.wasmmetadataconfig_set_extract_links(this.__wbg_ptr, value);
  }
  /**
   * @param {boolean} value
   */
  set extract_images(value) {
    wasm.wasmmetadataconfig_set_extract_images(this.__wbg_ptr, value);
  }
  /**
   * @param {boolean} value
   */
  set extract_headers(value) {
    wasm.wasmmetadataconfig_set_extract_headers(this.__wbg_ptr, value);
  }
  /**
   * @param {boolean} value
   */
  set extract_document(value) {
    wasm.wasmmetadataconfig_set_extract_document(this.__wbg_ptr, value);
  }
  /**
   * @returns {boolean}
   */
  get extract_structured_data() {
    const ret = wasm.wasmmetadataconfig_extract_structured_data(this.__wbg_ptr);
    return ret !== 0;
  }
  /**
   * @returns {number}
   */
  get max_structured_data_size() {
    const ret =
        wasm.wasmmetadataconfig_max_structured_data_size(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * @param {boolean} value
   */
  set extract_structured_data(value) {
    wasm.wasmmetadataconfig_set_extract_structured_data(this.__wbg_ptr, value);
  }
  /**
   * @param {number} value
   */
  set max_structured_data_size(value) {
    wasm.wasmmetadataconfig_set_max_structured_data_size(this.__wbg_ptr, value);
  }
  /**
   * Create a new metadata configuration with defaults
   *
   * All extraction types enabled by default with 1MB structured data limit
   */
  constructor() {
    const ret = wasm.wasmmetadataconfig_new();
    this.__wbg_ptr = ret >>> 0;
    WasmMetadataConfigFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
}
if (Symbol.dispose)
  WasmMetadataConfig.prototype[Symbol.dispose] =
      WasmMetadataConfig.prototype.free;

/**
 * Convert HTML to Markdown
 *
 * # Arguments
 *
 * * `html` - The HTML string to convert
 * * `options` - Optional conversion options (as a JavaScript object)
 *
 * # Example
 *
 * ```javascript
 * import { convert } from 'html-to-markdown-wasm';
 *
 * const html = '<h1>Hello World</h1>';
 * const markdown = convert(html);
 * console.log(markdown); // # Hello World
 * ```
 * @param {string} html
 * @param {any} options
 * @returns {string}
 */
export function convert(html, options) {
  let deferred3_0;
  let deferred3_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_export,
                                   wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    wasm.convert(retptr, ptr0, len0, addHeapObject(options));
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
    var ptr2 = r0;
    var len2 = r1;
    if (r3) {
      ptr2 = 0;
      len2 = 0;
      throw takeObject(r2);
    }
    deferred3_0 = ptr2;
    deferred3_1 = len2;
    return getStringFromWasm0(ptr2, len2);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred3_0, deferred3_1, 1);
  }
}

/**
 * @param {Uint8Array} html
 * @param {any} options
 * @returns {string}
 */
export function convertBytes(html, options) {
  let deferred2_0;
  let deferred2_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    wasm.convertBytes(retptr, addHeapObject(html), addHeapObject(options));
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
    var ptr1 = r0;
    var len1 = r1;
    if (r3) {
      ptr1 = 0;
      len1 = 0;
      throw takeObject(r2);
    }
    deferred2_0 = ptr1;
    deferred2_1 = len1;
    return getStringFromWasm0(ptr1, len1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
  }
}

/**
 * @param {Uint8Array} html
 * @param {any} options
 * @param {WasmInlineImageConfig | null} [image_config]
 * @returns {WasmHtmlExtraction}
 */
export function convertBytesWithInlineImages(html, options, image_config) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    let ptr0 = 0;
    if (!isLikeNone(image_config)) {
      _assertClass(image_config, WasmInlineImageConfig);
      ptr0 = image_config.__destroy_into_raw();
    }
    wasm.convertBytesWithInlineImages(retptr, addHeapObject(html),
                                      addHeapObject(options), ptr0);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return WasmHtmlExtraction.__wrap(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Convert HTML bytes to Markdown with metadata extraction
 *
 * # Arguments
 *
 * * `html` - The HTML bytes to convert
 * * `options` - Optional conversion options (as a JavaScript object)
 * * `metadata_config` - Metadata extraction configuration
 *
 * # Returns
 *
 * JavaScript object with `markdown` (string) and `metadata` (object) fields
 * @param {Uint8Array} html
 * @param {any} options
 * @param {WasmMetadataConfig | null} [metadata_config]
 * @returns {any}
 */
export function convertBytesWithMetadata(html, options, metadata_config) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    let ptr0 = 0;
    if (!isLikeNone(metadata_config)) {
      _assertClass(metadata_config, WasmMetadataConfig);
      ptr0 = metadata_config.__destroy_into_raw();
    }
    wasm.convertBytesWithMetadata(retptr, addHeapObject(html),
                                  addHeapObject(options), ptr0);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * @param {Uint8Array} html
 * @param {WasmConversionOptionsHandle} handle
 * @returns {string}
 */
export function convertBytesWithOptionsHandle(html, handle) {
  let deferred2_0;
  let deferred2_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    _assertClass(handle, WasmConversionOptionsHandle);
    wasm.convertBytesWithOptionsHandle(retptr, addHeapObject(html),
                                       handle.__wbg_ptr);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
    var ptr1 = r0;
    var len1 = r1;
    if (r3) {
      ptr1 = 0;
      len1 = 0;
      throw takeObject(r2);
    }
    deferred2_0 = ptr1;
    deferred2_1 = len1;
    return getStringFromWasm0(ptr1, len1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
  }
}

/**
 * @param {string} html
 * @param {any} options
 * @param {WasmInlineImageConfig | null} [image_config]
 * @returns {WasmHtmlExtraction}
 */
export function convertWithInlineImages(html, options, image_config) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_export,
                                   wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    let ptr1 = 0;
    if (!isLikeNone(image_config)) {
      _assertClass(image_config, WasmInlineImageConfig);
      ptr1 = image_config.__destroy_into_raw();
    }
    wasm.convertWithInlineImages(retptr, ptr0, len0, addHeapObject(options),
                                 ptr1);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return WasmHtmlExtraction.__wrap(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Convert HTML to Markdown with metadata extraction
 *
 * # Arguments
 *
 * * `html` - The HTML string to convert
 * * `options` - Optional conversion options (as a JavaScript object)
 * * `metadata_config` - Metadata extraction configuration
 *
 * # Returns
 *
 * JavaScript object with `markdown` (string) and `metadata` (object) fields
 *
 * # Example
 *
 * ```javascript
 * import { convertWithMetadata, WasmMetadataConfig } from
 * 'html-to-markdown-wasm';
 *
 * const html = '<h1>Hello World</h1><a href="https://example.com">Link</a>';
 * const config = new WasmMetadataConfig();
 * config.extractHeaders = true;
 * config.extractLinks = true;
 *
 * const result = convertWithMetadata(html, null, config);
 * console.log(result.markdown); // # Hello World\n\n[Link](https://example.com)
 * console.log(result.metadata.headers); // [{ level: 1, text: "Hello World",
 * ... }] console.log(result.metadata.links); // [{ href: "https://example.com",
 * text: "Link", ... }]
 * ```
 * @param {string} html
 * @param {any} options
 * @param {WasmMetadataConfig | null} [metadata_config]
 * @returns {any}
 */
export function convertWithMetadata(html, options, metadata_config) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_export,
                                   wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    let ptr1 = 0;
    if (!isLikeNone(metadata_config)) {
      _assertClass(metadata_config, WasmMetadataConfig);
      ptr1 = metadata_config.__destroy_into_raw();
    }
    wasm.convertWithMetadata(retptr, ptr0, len0, addHeapObject(options), ptr1);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return takeObject(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * @param {string} html
 * @param {WasmConversionOptionsHandle} handle
 * @returns {string}
 */
export function convertWithOptionsHandle(html, handle) {
  let deferred3_0;
  let deferred3_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(html, wasm.__wbindgen_export,
                                   wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    _assertClass(handle, WasmConversionOptionsHandle);
    wasm.convertWithOptionsHandle(retptr, ptr0, len0, handle.__wbg_ptr);
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
    var ptr2 = r0;
    var len2 = r1;
    if (r3) {
      ptr2 = 0;
      len2 = 0;
      throw takeObject(r2);
    }
    deferred3_0 = ptr2;
    deferred3_1 = len2;
    return getStringFromWasm0(ptr2, len2);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_export4(deferred3_0, deferred3_1, 1);
  }
}

/**
 * @param {any} options
 * @returns {WasmConversionOptionsHandle}
 */
export function createConversionOptionsHandle(options) {
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    wasm.createConversionOptionsHandle(retptr, addHeapObject(options));
    var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
    var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
    var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
    if (r2) {
      throw takeObject(r1);
    }
    return WasmConversionOptionsHandle.__wrap(r0);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
  }
}

/**
 * Initialize panic hook for better error messages in the browser
 */
export function init() { wasm.init(); }

const EXPECTED_RESPONSE_TYPES = new Set([ "basic", "cors", "default" ]);

async function __wbg_load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        const validResponse =
            module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

        if (validResponse &&
            module.headers.get("Content-Type") !== "application/wasm") {
          console.warn(
              "`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",
              e,
          );
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
      return {instance, module};
    } else {
      return instance;
    }
  }
}

function __wbg_get_imports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbg_Error_52673b7de5a0ca89 = function(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_Number_2d1dcfcf4ec51736 = function(arg0) {
    const ret = Number(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
    const ret = String(getObject(arg1));
    const ptr1 =
        passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_bigint_get_as_i64_6e32f5e6aff02e1d = function(
      arg0, arg1) {
    const v = getObject(arg1);
    const ret = typeof v === "bigint" ? v : undefined;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1,
                                     isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbg___wbindgen_boolean_get_dea25b33882b895b = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof v === "boolean" ? v : undefined;
    return isLikeNone(ret) ? 0xffffff : ret ? 1 : 0;
  };
  imports.wbg.__wbg___wbindgen_debug_string_adfb662ae34724b6 = function(arg0,
                                                                        arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 =
        passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_in_0d3e1e8f0c669317 = function(arg0, arg1) {
    const ret = getObject(arg0) in getObject(arg1);
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_bigint_0e1a2e3f55cfae27 = function(arg0) {
    const ret = typeof getObject(arg0) === "bigint";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
    const ret = typeof getObject(arg0) === "function";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_null_dfda7d66506c95b5 = function(arg0) {
    const ret = getObject(arg0) === null;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
    const val = getObject(arg0);
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
    const ret = typeof getObject(arg0) === "string";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_jsval_eq_b6101cc9cef1fe36 = function(arg0,
                                                                    arg1) {
    const ret = getObject(arg0) === getObject(arg1);
    return ret;
  };
  imports.wbg.__wbg___wbindgen_jsval_loose_eq_766057600fdd1b0d = function(
      arg0, arg1) {
    const ret = getObject(arg0) == getObject(arg1);
    return ret;
  };
  imports.wbg.__wbg___wbindgen_number_get_9619185a74197f95 = function(arg0,
                                                                      arg1) {
    const obj = getObject(arg1);
    const ret = typeof obj === "number" ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret,
                                    true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbg___wbindgen_string_get_a2a31e16edf96e42 = function(arg0,
                                                                      arg1) {
    const obj = getObject(arg1);
    const ret = typeof obj === "string" ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0
                               : passStringToWasm0(ret, wasm.__wbindgen_export,
                                                   wasm.__wbindgen_export2);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(
      arg0, arg1) { throw new Error(getStringFromWasm0(arg0, arg1)); };
  imports.wbg.__wbg_call_abb4ff46ce38be40 = function() {
    return handleError(function(arg0, arg1) {
      const ret = getObject(arg0).call(getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_codePointAt_6fd4439a1e465afd = function(arg0, arg1) {
    const ret = getObject(arg0).codePointAt(arg1 >>> 0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_done_62ea16af4ce34b24 = function(arg0) {
    const ret = getObject(arg0).done;
    return ret;
  };
  imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
      deferred0_0 = arg0;
      deferred0_1 = arg1;
      console.error(getStringFromWasm0(arg0, arg1));
    } finally {
      wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
    }
  };
  imports.wbg.__wbg_get_6b7bd52aca3f9671 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_get_af9dab7e9603ea93 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.get(getObject(arg0), getObject(arg1));
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_get_with_ref_key_1dc361bd10053bfe = function(arg0, arg1) {
    const ret = getObject(arg0)[getObject(arg1)];
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_instanceof_ArrayBuffer_f3320d2419cd0355 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof ArrayBuffer;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Object_577e21051f7bcb79 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof Object;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Uint8Array_da54ccc9d3e09434 = function(arg0) {
    let result;
    try {
      result = getObject(arg0) instanceof Uint8Array;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_isArray_51fd9e6422c0a395 = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbg_isSafeInteger_ae7d3f054d55fa16 = function(arg0) {
    const ret = Number.isSafeInteger(getObject(arg0));
    return ret;
  };
  imports.wbg.__wbg_iterator_27b7c8b35ab3e86b = function() {
    const ret = Symbol.iterator;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_keys_f5c6002ff150fc6c = function(arg0) {
    const ret = Object.keys(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_length_1f83b8e5895c84aa = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_length_22ac23eaec9d8053 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_length_d45040a40c570362 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
  };
  imports.wbg.__wbg_new_1ba21ce319a06297 = function() {
    const ret = new Object();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_25f239778d6112b9 = function() {
    const ret = new Array();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_6421f6084cc5bc5a = function(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
    const ret = new Error();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_b546ae120718850e = function() {
    const ret = new Map();
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_new_from_slice_f9c22b9153b26992 = function(arg0, arg1) {
    const ret = new Uint8Array(getArrayU8FromWasm0(arg0, arg1));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_next_138a17bbf04e926c = function(arg0) {
    const ret = getObject(arg0).next;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_next_3cfe5c0fe2a4cc53 = function() {
    return handleError(function(arg0) {
      const ret = getObject(arg0).next();
      return addHeapObject(ret);
    }, arguments);
  };
  imports.wbg.__wbg_prototypesetcall_dfe9b766cdc1f1fd = function(arg0, arg1,
                                                                 arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1),
                                  getObject(arg2));
  };
  imports.wbg.__wbg_set_3f1d0b984ed272ed = function(
      arg0, arg1,
      arg2) { getObject(arg0)[takeObject(arg1)] = takeObject(arg2); };
  imports.wbg.__wbg_set_781438a03c0c3c81 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret =
          Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_set_7df433eea03a5c14 = function(
      arg0, arg1, arg2) { getObject(arg0)[arg1 >>> 0] = takeObject(arg2); };
  imports.wbg.__wbg_set_efaaf145b9377369 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 =
        passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg_value_57b7b035e117f7ee = function(arg0) {
    const ret = getObject(arg0).value;
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_wasminlineimage_new = function(arg0) {
    const ret = WasmInlineImage.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbg_wasminlineimagewarning_new = function(arg0) {
    const ret = WasmInlineImageWarning.__wrap(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_cast_4625c577ab2ec9ee = function(arg0) {
    // Cast intrinsic for `U64 -> Externref`.
    const ret = BigInt.asUintN(64, arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
  };
  imports.wbg.__wbindgen_object_drop_ref = function(arg0) { takeObject(arg0); };

  return imports;
}

function __wbg_finalize_init(instance, module) {
  wasm = instance.exports;
  __wbg_init.__wbindgen_wasm_module = module;
  cachedDataViewMemory0 = null;
  cachedUint32ArrayMemory0 = null;
  cachedUint8ArrayMemory0 = null;

  wasm.__wbindgen_start();
  return wasm;
}

function initSync(module) {
  if (wasm !== undefined)
    return wasm;

  if (typeof module !== "undefined") {
    if (Object.getPrototypeOf(module) === Object.prototype) {
      ({module} = module);
    } else {
      console.warn(
          "using deprecated parameters for `initSync()`; pass a single object instead");
    }
  }

  const imports = __wbg_get_imports();
  if (!(module instanceof WebAssembly.Module)) {
    module = new WebAssembly.Module(module);
  }
  const instance = new WebAssembly.Instance(module, imports);
  return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
  if (wasm !== undefined)
    return wasm;

  if (typeof module_or_path !== "undefined") {
    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
      ({module_or_path} = module_or_path);
    } else {
      console.warn(
          "using deprecated parameters for the initialization function; pass a single object instead");
    }
  }

  if (typeof module_or_path === "undefined") {
    module_or_path = new URL("html_to_markdown_wasm_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();

  if (typeof module_or_path === "string" ||
      (typeof Request === "function" && module_or_path instanceof Request) ||
      (typeof URL === "function" && module_or_path instanceof URL)) {
    module_or_path = fetch(module_or_path);
  }

  const {instance, module} = await __wbg_load(await module_or_path, imports);

  return __wbg_finalize_init(instance, module);
}

export {initSync};
export default __wbg_init;
