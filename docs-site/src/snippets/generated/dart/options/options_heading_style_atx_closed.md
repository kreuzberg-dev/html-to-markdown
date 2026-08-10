```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"heading_style":"AtxClosed"}');
  final result = await H2mBridge.convert('<h1>Closed Heading</h1>', options: _options);
}

```
