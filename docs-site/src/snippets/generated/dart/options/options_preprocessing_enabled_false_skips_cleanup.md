---
id: fixture_dart_options_preprocessing_enabled_false_skips_cleanup
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"enabled":false}}');
  final result = await H2mBridge.convert('<nav>NavSection</nav><p>Paragraph</p>', options: _options);
}

```
