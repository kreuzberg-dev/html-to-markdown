# html_to_markdown_rs

High-performance HTML to Markdown converter

## Installation

Install Zig from [ziglang.org](https://ziglang.org/download/).

## Building

```sh
zig build
zig build test
```

## Usage

Add to your `build.zig.zon`:

```text
.dependencies = .{
    .html_to_markdown_rs = .{
        .path = "path/to/html_to_markdown_rs",
    },
},
```

## License

MIT
