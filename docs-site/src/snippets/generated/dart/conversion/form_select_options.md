---
id: fixture_dart_form_select_options
language: dart
target: dart
level: typecheck
requires: []
side_effect: safe
---

```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_forms":false}}');
  final result = await H2mBridge.convert('<form><label>Color:</label><select><option value="red">Red</option><option value="blue" selected>Blue</option><option value="green">Green</option></select></form>', options: _options);
}

```
