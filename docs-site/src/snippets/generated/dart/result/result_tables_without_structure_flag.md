```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>', options: _options);
}

```
