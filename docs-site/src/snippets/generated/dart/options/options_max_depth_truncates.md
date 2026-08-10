```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"max_depth":3}');
  final result = await H2mBridge.convert('<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>', options: _options);
}

```
