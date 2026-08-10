```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<ol><li>Step 1<ol><li>Step 1a</li><li>Step 1b</li></ol></li><li>Step 2</li></ol>', options: _options);
}

```
