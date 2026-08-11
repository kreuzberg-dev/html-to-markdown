---
id: fixture_dart_link_with_bold_text
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
  final result = await H2mBridge.convert('<a href="https://example.com"><strong>Bold link</strong></a>', options: _options);
}

```
