---
id: fixture_dart_style_tags_only
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
  final result = await H2mBridge.convert('<html><head><style>body { color: red; }</style></head><body><style>.foo { margin: 0; }</style></body></html>', options: _options);
}

```
