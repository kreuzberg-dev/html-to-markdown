```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"escape_asterisks":true}');
  final result = await H2mBridge.convert('<p>Use 2*3 = 6 in math.</p>', options: _options);
}

```
