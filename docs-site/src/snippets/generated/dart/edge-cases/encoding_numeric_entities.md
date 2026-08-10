```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<p>Copyright: &#169; Trade: &#174; Euro: &#8364; Hex: &#x00A9;</p>', options: _options);
}

```
