---
id: fixture_dart_options_url_escape_style_angle_default
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"url_escape_style":"angle"}');
  final result = await H2mBridge.convert('<a href="/file (1).pdf">file</a>', options: _options);
}

```
