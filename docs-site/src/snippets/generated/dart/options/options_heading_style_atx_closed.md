---
id: fixture_dart_options_heading_style_atx_closed
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"heading_style":"AtxClosed"}');
  final result = await H2mBridge.convert('<h1>Closed Heading</h1>', options: _options);
}

```
