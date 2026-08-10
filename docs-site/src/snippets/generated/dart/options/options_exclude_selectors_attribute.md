```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"exclude_selectors":["[role=\'complementary\']"]}');
  final result = await H2mBridge.convert('<body><div role="complementary">Sidebar</div><p>Primary text</p></body>', options: _options);
}

```
