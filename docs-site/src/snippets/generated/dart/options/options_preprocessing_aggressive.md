```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"preset":"Aggressive"}}');
  final result = await H2mBridge.convert('<nav>Menu</nav><article><h1>Title</h1><p>Content</p></article><aside>Sidebar</aside><footer>Footer</footer>', options: _options);
}

```
