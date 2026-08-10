```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_forms":true}}');
  final result = await H2mBridge.convert('<p>Before</p><form><input type=\'text\'/><button>Submit</button></form><p>After</p>', options: _options);
}

```
