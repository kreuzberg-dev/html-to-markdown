---
id: fixture_dart_structured_data_json_ld
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
  final result = await H2mBridge.convert('<html><head><title>Article</title><script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","headline":"My Article","author":{"@type":"Person","name":"Jane Doe"},"datePublished":"2024-01-15"}</script></head><body><h1>My Article</h1><p>Article body text.</p></body></html>', options: _options);
}

```
