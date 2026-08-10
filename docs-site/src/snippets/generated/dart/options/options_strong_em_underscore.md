```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"strong_em_symbol":"_"}');
  final result = await H2mBridge.convert('<p><strong>bold</strong> and <em>italic</em></p>', options: _options);
}

```
