---
id: fixture_dart_options_exclude_selectors_multiple
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":[".nav","footer"]}');
  final result = await H2mBridge.convert('<body><nav class="nav">Menu</nav><p>Content</p><footer>Footer</footer></body>', options: _options);
}

```
