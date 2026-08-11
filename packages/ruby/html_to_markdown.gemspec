# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "html-to-markdown"
  spec.version = "3.10.6"
  spec.authors       = ["Na'aman Hirschfeld <naaman@xberg.io>"]
  spec.summary       = "High-performance HTML to Markdown converter"
  spec.description   = "High-performance HTML to Markdown converter"
  spec.homepage      = "https://github.com/xberg-io/html-to-markdown"

  spec.license       = "MIT"

  spec.required_ruby_version = ">= 3.2.0"
  spec.metadata["keywords"] = %w[converter html markdown].join(",")
  spec.metadata["rubygems_mfa_required"] = "true"

  # ~keep A source gem must ship only sources: `gem build` runs against a working tree that
  # may already hold a compiled extension, and any native artifact swept in here would be
  # installed as-is on machines of a different platform, shadowing the one `gem install`
  # compiles. Reject build output rather than enumerating sources so a new source file is
  # picked up automatically.
  build_artifacts    = %r{/(?:target|tmp)/|\.(?:bundle|so|dylib|dll|o|a|log)\z|\.dSYM/|(?:\A|/)Makefile\z}
  candidate_files    = Dir.glob(%w[README* LICENSE* lib/**/* ext/**/* sig/**/* Steepfile]).select { |f| File.file?(f) }
  spec.files         = candidate_files.reject { |f| f.match?(build_artifacts) }
  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/html_to_markdown_rb/native/extconf.rb"]

  spec.add_dependency "rb_sys", ">= 0.9", "< 0.9.128"
  spec.add_dependency "sorbet-runtime", "~> 0.5"
end
