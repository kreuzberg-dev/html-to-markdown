---
id: fixture_dart_metadata_extract_all_links
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
  final result = await H2mBridge.convert('<html><head><title>Links Page</title></head><body><p>Visit <a href="https://example.com">Example</a> or <a href="https://docs.example.com">Docs</a>.</p><p>Also see <a href="/relative/path">relative link</a> and <a href="mailto:hello@example.com">email us</a>.</p></body></html>', options: _options);
}

```
