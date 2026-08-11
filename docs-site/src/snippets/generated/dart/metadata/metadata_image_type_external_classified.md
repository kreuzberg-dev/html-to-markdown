---
id: fixture_dart_metadata_image_type_external_classified
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<p><img src="https://example.com/photo.jpg" alt="A photo"></p>', options: _options);
}

```
