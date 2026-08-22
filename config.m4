dnl Configuration for Rust-based PHP extension via ext-php-rs.
dnl Allows phpize to recognize this extension during source compilation (PIE fallback).

PHP_ARG_ENABLE([html_to_markdown],
  [whether to enable the html_to_markdown extension],
  [AS_HELP_STRING([--enable-html_to_markdown],
    [Enable html_to_markdown extension support])],
  [yes])

if test "$PHP_HTML_TO_MARKDOWN_ENABLED" = "yes"; then
  dnl Register the extension directory so phpize creates modules/ and sets up build rules.
  PHP_NEW_EXTENSION(html_to_markdown, [], $ext_shared)

  dnl Invoke cargo build to compile the Rust FFI library and copy it to modules/.
  AC_CONFIG_COMMANDS([cargo-build], [
    if test -f "crates/html-to-markdown-rs-php/Cargo.toml"; then
      (cd crates/html-to-markdown-rs-php && cargo build --release) || exit 1

      dnl Detect output filename based on platform
      if test -f "crates/html-to-markdown-rs-php/target/release/libhtml_to_markdown_rs_php.dylib"; then
        cargo_lib="crates/html-to-markdown-rs-php/target/release/libhtml_to_markdown_rs_php.dylib"
      elif test -f "crates/html-to-markdown-rs-php/target/release/libhtml_to_markdown_rs_php.so"; then
        cargo_lib="crates/html-to-markdown-rs-php/target/release/libhtml_to_markdown_rs_php.so"
      else
        echo "ERROR: cargo build succeeded but .so/.dylib not found in crates/html-to-markdown-rs-php/target/release" >&2
        exit 1
      fi

      mkdir -p modules
      cp "$cargo_lib" "modules/html_to_markdown.so" || exit 1
    else
      echo "ERROR: crates/html-to-markdown-rs-php/Cargo.toml not found" >&2
      exit 1
    fi
  ], [])
fi
