```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{"preprocessing":{"remove_navigation":false}}');
  final result = await H2mBridge.convert('<nav>SiteMenu</nav><main><p>MainContent</p></main><aside>SidebarText</aside>', options: _options);
}

```
