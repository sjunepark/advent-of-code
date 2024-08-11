default:
    just --list

watch day="" part="":
    cargo watch -q -c \
    {{if day != "" {"-w day-" + day} else {""} }} \
    -s "just test {{day}} {{part}}"
    -x 'fmt --all -- --check'

watch-all:
    just watch

lint day:
    cargo clippy --all-features

test day="" name="":
    cargo test --all-features \
    {{if day != "" {"-p day-" + day} else {""} }} \
    {{if name != "" {name} else {""} }}