```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"capture_svg":true,"extract_images":true}');
  final result = await H2mBridge.convert('<p>Below SVG:</p><svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect width="10" height="10" fill="red"/></svg>', options: _options);
}

```
