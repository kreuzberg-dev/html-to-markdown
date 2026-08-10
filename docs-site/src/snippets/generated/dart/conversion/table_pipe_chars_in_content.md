```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<table><thead><tr><th>Expression</th><th>Result</th></tr></thead><tbody><tr><td>a | b</td><td>true</td></tr></tbody></table>', options: _options);
}

```
