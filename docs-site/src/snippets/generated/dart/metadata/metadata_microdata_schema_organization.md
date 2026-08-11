---
id: fixture_dart_metadata_microdata_schema_organization
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
  final result = await H2mBridge.convert('<html><head><title>Company</title></head><body><div itemscope itemtype="https://schema.org/Organization"><span itemprop="name">Acme Corp</span><span itemprop="foundingDate">2020</span><span itemprop="url">https://acmecorp.example.com</span><span itemprop="logo">https://acmecorp.example.com/logo.png</span></div></body></html>', options: _options);
}

```
