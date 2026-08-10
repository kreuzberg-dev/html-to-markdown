```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html lang="en" dir="ltr"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>', options: _options);
}

```
