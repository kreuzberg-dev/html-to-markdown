```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"output_format":"Djot"}');
  final result = await H2mBridge.convert('<p>Simple paragraph.</p>', options: _options);
}

```
