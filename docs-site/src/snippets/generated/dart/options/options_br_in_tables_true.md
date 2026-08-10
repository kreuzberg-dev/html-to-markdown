```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"br_in_tables":true}');
  final result = await H2mBridge.convert('<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>', options: _options);
}

```
