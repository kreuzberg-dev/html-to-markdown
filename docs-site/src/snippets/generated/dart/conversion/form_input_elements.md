```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_forms":false}}');
  final result = await H2mBridge.convert('<form><label for="name">Name:</label><input type="text" id="name" placeholder="Enter name"></form>', options: _options);
}

```
