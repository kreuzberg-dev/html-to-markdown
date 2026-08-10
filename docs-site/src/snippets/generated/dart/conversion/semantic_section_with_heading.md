```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<section><h3>Section Heading</h3><p>Section content.</p></section>', options: _options);
}

```
