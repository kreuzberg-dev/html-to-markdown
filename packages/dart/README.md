# h2m

High-performance HTML to Markdown converter

## Installation

Add to your `pubspec.yaml`:

```yaml
dependencies:
  h2m: ^3.10.6
```

Then run:

```sh
dart pub get
```

## Building

From the repository root:

```sh
cargo build -p html-to-markdown-rs-dart
flutter_rust_bridge_codegen generate
dart pub get
dart analyze
dart test
```

## License

MIT
