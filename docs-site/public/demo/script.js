import init, {
  convert,
} from "https://cdn.jsdelivr.net/npm/@xberg-io/html-to-markdown-wasm@latest/pkg/web/html_to_markdown_wasm.js";

let wasmInitialized = false;

const htmlInput = document.getElementById("htmlInput");
const outputMarkdown = document.getElementById("outputMarkdown");
const outputJson = document.getElementById("outputJson");
const copyBtn = document.getElementById("copyBtn");
const clearBtn = document.getElementById("clearBtn");
const statusEl = document.getElementById("status");
const tabMarkdown = document.getElementById("tabMarkdown");
const tabJson = document.getElementById("tabJson");
const tabIndicator = document.getElementById("tabIndicator");

let activeTab = "markdown";

async function initWasm() {
  try {
    await init();
    wasmInitialized = true;
    statusEl.textContent = "Ready";
    statusEl.className = "status-text success";
    performConversion();
  } catch (error) {
    console.error("Failed to initialize WASM:", error);
    statusEl.textContent = "Failed to load WASM module";
    statusEl.className = "status-text error";
  }
}

function highlightJson(obj) {
  const raw = JSON.stringify(obj, null, 2);
  return raw.replace(
    /("[^"]*"\s*:)|("[^"]*")|(-?\d+\.?\d*(?:[eE][+-]?\d+)?)|(true|false)|(null)/g,
    (match, key, str, num, bool, nil) => {
      if (key) return `<span class="json-key">${key}</span>`;
      if (str) return `<span class="json-string">${str}</span>`;
      if (num) return `<span class="json-number">${num}</span>`;
      if (bool) return `<span class="json-bool">${bool}</span>`;
      if (nil) return `<span class="json-null">${nil}</span>`;
      return match;
    },
  );
}

function performConversion() {
  if (!wasmInitialized) {
    statusEl.textContent = "WASM module not initialized yet...";
    statusEl.className = "status-text";
    return;
  }

  const html = htmlInput.value.trim();

  if (!html) {
    outputMarkdown.textContent = "";
    outputJson.innerHTML = "";
    statusEl.textContent = "Enter some HTML to convert";
    statusEl.className = "status-text";
    return;
  }

  try {
    const startTime = performance.now();
    const result = convert(html, null);
    const duration = (performance.now() - startTime).toFixed(2);

    outputMarkdown.textContent = result.content ?? "";

    const jsonData = {
      content: result.content ?? null,
      metadata: result.metadata ?? null,
      tables: result.tables ?? [],
      images: (result.images ?? []).map((img) => ({
        format: img.format,
        filename: img.filename ?? null,
        description: img.description ?? null,
        width: img.width ?? null,
        height: img.height ?? null,
        source: img.source,
      })),
      warnings: result.warnings ?? [],
    };
    outputJson.innerHTML = highlightJson(jsonData);

    statusEl.textContent = `Converted in ${duration}ms`;
    statusEl.className = "status-text success";
    copyBtn.classList.remove("copied");
    copyBtn.textContent = "Copy";
  } catch (error) {
    console.error("Conversion error:", error);
    outputMarkdown.textContent = "";
    outputJson.innerHTML = "";
    statusEl.textContent = `Error: ${error.message || error}`;
    statusEl.className = "status-text error";
  }
}

function switchTab(tab) {
  activeTab = tab;
  const isMd = tab === "markdown";
  tabIndicator.classList.toggle("json", !isMd);
  tabMarkdown.classList.toggle("active", isMd);
  tabJson.classList.toggle("active", !isMd);
  outputMarkdown.classList.toggle("hidden", !isMd);
  outputJson.classList.toggle("hidden", isMd);
}

async function copyToClipboard() {
  const text = activeTab === "markdown" ? outputMarkdown.textContent : outputJson.innerText;

  if (!text) {
    statusEl.textContent = "Nothing to copy";
    statusEl.className = "status-text";
    return;
  }

  try {
    await navigator.clipboard.writeText(text);
    copyBtn.classList.add("copied");
    copyBtn.textContent = "Copied!";
    statusEl.textContent = "Copied to clipboard";
    statusEl.className = "status-text success";
    setTimeout(() => {
      copyBtn.classList.remove("copied");
      copyBtn.textContent = "Copy";
    }, 2000);
  } catch (error) {
    console.error("Failed to copy:", error);
    statusEl.textContent = "Failed to copy to clipboard";
    statusEl.className = "status-text error";
  }
}

function clearInput() {
  htmlInput.value = "";
  outputMarkdown.textContent = "";
  outputJson.innerHTML = "";
  statusEl.textContent = "Input cleared";
  statusEl.className = "status-text";
  htmlInput.focus();
}

tabMarkdown.addEventListener("click", () => switchTab("markdown"));
tabJson.addEventListener("click", () => switchTab("json"));
copyBtn.addEventListener("click", copyToClipboard);
clearBtn.addEventListener("click", clearInput);

htmlInput.addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key === "Enter") {
    e.preventDefault();
    performConversion();
  }
});

let debounceTimer;
htmlInput.addEventListener("input", () => {
  clearTimeout(debounceTimer);
  debounceTimer = setTimeout(() => {
    if (wasmInitialized && htmlInput.value.trim()) {
      performConversion();
    }
  }, 300);
});

initWasm();
