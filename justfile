clean:
  clear
build: clean
  cargo build
test: clean
  cargo test
clippy: clean
  cargo clippy
check: clean
  cargo check
install: clean
  cargo install --path .
run-help: build clean
  ./target/debug/pastol -h
run-listed-true: build clean
  ./target/debug/pastol --setunlist true
run-listed-false: build clean
  ./target/debug/pastol --setunlist false
run-example: build clean
  ./target/debug/pastol -t example -c "Example content."
run-example-file: build clean
  ./target/debug/pastol -f example.md
run-example-no-title: build clean
  ./target/debug/pastol -c example
run-example-no-content: build clean
  ./target/debug/pastol -t example
run-exaple-remove: run-example clean
  ./target/debug/pastol -r example
run-exaple-remove-fail: build clean
  ./target/debug/pastol -r exampledwadawdas
run-list: build clean
  ./target/debug/pastol --list
run-list-fail: build clean
  ./target/debug/pastol --list
run-info: build clean
  ./target/debug/pastol --info sign-git-commits-with-gpg-in-fish-shell
run-info-fail: build clean
  ./target/debug/pastol --info sign-git-commits-with-gpg-in-fish-shelldwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww
run-download: build clean
  ./target/debug/pastol --download sign-git-commits-with-gpg-in-fish-shell
run-download-fail: build clean
  ./target/debug/pastol --download sign-git-commits-with-gpg-in-fish-shelldwadwwwwwwwwwwwwwwwwwwwwwww