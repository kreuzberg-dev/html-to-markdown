```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"debug":true}');
  final result = await H2mBridge.convert('<p>Debug test</p>', options: _options);
}

```
