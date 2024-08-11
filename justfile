default:
    just --list

watch day="":
    cargo watch -q -c \
    {{if day != "" {"-w day-" + day} else {""} }} \
    -s "just test {{day}}"
    -x 'fmt --all -- --check'

watch-all:
    just watch

test-watch day="" name="":
    cargo watch -q -c \
    {{if day != "" {"-w day-" + day} else {""} }} \
    -s "just test {{day}} {{name}}"

test day="" name="":
    cargo test --all-features \
    {{if day != "" {"-p day-" + day} else {""} }} \
    {{if name != "" {name} else {""} }} \
    -- --nocapture

check:
    cargo fmt --all
    cargo clippy --all-features
    cargo check --all-features
