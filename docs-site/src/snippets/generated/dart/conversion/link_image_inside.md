```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<a href="https://example.com"><img src="logo.png" alt="Logo"></a>', options: _options);
}

```
