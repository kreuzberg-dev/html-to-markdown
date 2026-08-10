```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"extract_metadata":true}');
  final result = await H2mBridge.convert('<html lang="ar" dir="rtl"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>', options: _options);
}

```
