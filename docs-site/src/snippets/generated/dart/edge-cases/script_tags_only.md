```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<html><head><script>alert(\'xss\')</script></head><body><script>document.write(\'hello\')</script></body></html>', options: _options);
}

```
