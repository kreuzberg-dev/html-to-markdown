---
id: fixture_dart_metadata_microdata_schema_person
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
  final result = await H2mBridge.convert('<html><head><title>Contact</title></head><body><div itemscope itemtype="https://schema.org/Person"><span itemprop="name">John Smith</span><span itemprop="email">john@example.com</span><span itemprop="telephone">+1-555-0100</span></div></body></html>', options: _options);
}

```
