// swift-tools-version: 6.0
// The first-party dependency pin below is managed by alef (sync.text_replacements); do not edit it by hand.
// alef:hash:1d960d1b0c0133a3ff0d7b28e2558a923860347b34757cf63243613a7b1b5566
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(url: "https://github.com/xberg-io/html-to-markdown", branch: "release/swift/3.12.0"),
  ],
  targets: [
    .testTarget(
      name: "HtmlToMarkdownE2ETests",
      dependencies: [.product(name: "HtmlToMarkdown", package: "html-to-markdown")]
    ),
  ]
)
