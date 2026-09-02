#!/usr/bin/env bash
set -euo pipefail

if [[ -n "${EXTENSION_PATH:-}" ]]; then
  ini_file="$(mktemp)"
  echo "extension=${EXTENSION_PATH}" >"${ini_file}"
  export PHPRC="${ini_file}"
fi

# alef stopped emitting crates/html-to-markdown-php/src/composer.json in 1d1b0fbfcc
# (one manifest per layout since alef c159e2dc0), so `composer run test` from that
# directory now dies with "Composer could not find a composer.json file". The root
# manifest is the only one left and its PSR-4 map already points at that directory,
# so drive phpunit from the root against the package's own config -- phpunit resolves
# `<directory>tests</directory>` relative to the config file, not the cwd. ~keep
php vendor/bin/phpunit --configuration crates/html-to-markdown-php/src/phpunit.xml
