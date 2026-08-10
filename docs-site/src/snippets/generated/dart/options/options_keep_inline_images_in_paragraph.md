```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"keep_inline_images_in":["p"]}');
  final result = await H2mBridge.convert('<p>Text <img src=\'icon.png\' alt=\'icon\'> more text</p>', options: _options);
}

```
