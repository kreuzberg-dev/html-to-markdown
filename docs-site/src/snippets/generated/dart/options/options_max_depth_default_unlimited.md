---
id: fixture_dart_options_max_depth_default_unlimited
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
  final result = await H2mBridge.convert('<div><div><div><div><p>Deep content</p></div></div></div></div>', options: _options);
}

```
