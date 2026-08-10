```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"max_depth":0}');
  final result = await H2mBridge.convert('<p>Hello</p>', options: _options);
}

```
