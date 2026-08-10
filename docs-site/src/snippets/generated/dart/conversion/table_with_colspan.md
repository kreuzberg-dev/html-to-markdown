```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<table><thead><tr><th colspan="2">Full Name</th></tr></thead><tbody><tr><td>John</td><td>Doe</td></tr></tbody></table>', options: _options);
}

```
