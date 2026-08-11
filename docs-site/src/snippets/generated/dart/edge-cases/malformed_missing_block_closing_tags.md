---
id: fixture_dart_malformed_missing_block_closing_tags
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<div><h1>Title<p>First paragraph<p>Second paragraph</div>', options: _options);
}

```
