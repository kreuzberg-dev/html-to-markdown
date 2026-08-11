---
id: fixture_dart_hidden_content_display_none_dropped
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
  final result = await H2mBridge.convert('<p>visible</p><div style="display:none">secret hidden text</div><p>also visible</p>', options: _options);
}

```
