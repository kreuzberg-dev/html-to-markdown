```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"bullets":"*"}');
  final result = await H2mBridge.convert('<ul><li>Item A</li><li>Item B</li></ul>', options: _options);
}

```
