---
id: fixture_dart_xss_onclick_handler_removed
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
  final result = await H2mBridge.convert('<p><a href="https://example.com" onclick="alert(\'xss\')">Click me</a></p><button onmouseover="steal_data()">Hover me</button>', options: _options);
}

```
