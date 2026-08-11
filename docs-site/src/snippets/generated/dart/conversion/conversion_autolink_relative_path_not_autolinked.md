---
id: fixture_dart_conversion_autolink_relative_path_not_autolinked
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
  final result = await H2mBridge.convert('<a href="/docs/intro.html">/docs/intro.html</a>', options: _options);
}

```
