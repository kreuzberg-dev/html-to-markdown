---
id: fixture_dart_options_wrap_disabled
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"wrap":false}');
  final result = await H2mBridge.convert('<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>', options: _options);
}

```
