```dart title="Dart"
import 'package:h2m/html_to_markdown_rs.dart';
Future<void> main() async {
  final _options = await createConversionOptionsFromJson(json: '{}');
  final result = await H2mBridge.convert('<article><h1>Getting Started with Rust</h1><p>Rust is a systems programming language focused on <strong>safety</strong>, <em>performance</em>, and concurrency. It was created by <a href="https://www.mozilla.org">Mozilla</a> and has grown significantly in popularity.</p><h2>Installation</h2><p>Install Rust using the official installer:</p><pre><code class="language-bash">curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre><h2>Hello World</h2><p>Create your first Rust program:</p><pre><code class="language-rust">fn main() {\n    println!("Hello, world!");\n}</code></pre><p>Run it with <code>cargo run</code> from your project directory.</p><h2>Key Concepts</h2><ul><li>Ownership and borrowing</li><li>Lifetimes</li><li>Traits and generics</li><li>Pattern matching</li></ul><p>For more information, visit the <a href="https://doc.rust-lang.org/book/">Rust Book</a>.</p></article>', options: _options);
}

```
