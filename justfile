default:
    just --list

watch day="":
    cargo watch -q -c \
    {{if day != "" {"-w day-" + day} else {""} }} \
    -x 'check --all-features' \
    -s "just test {{day}}"
    -x 'fmt --all -- --check'

watch-all:
    just watch

lint day:
    cargo clippy --all-features

test day="" part="":
    cargo test --all-features \
    {{if day != "" {"-p day-" + day} else {""} }} \
    {{if part != "" {"--bin part" + part} else {""} }}