```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"strip_tags":["div","span"]}');
  final result = await H2mBridge.convert('<div class=\'wrapper\'><p>Inside div</p></div><p>Outside <span class=\'hl\'>span text</span></p>', options: _options);
}

```
