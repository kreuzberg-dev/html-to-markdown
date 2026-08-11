---
id: fixture_dart_options_url_escape_style_percent_image
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"url_escape_style":"percent"}');
  final result = await H2mBridge.convert('<img src="/img (1) <draft>.png" alt="alt">', options: _options);
}

```
