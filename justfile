
[private]
default:
    @just --list --unsorted

dev:
    cargo run

run +args="search":
    cargo run --release -- {{args}}

publish:
    ./scripts/publish.sh
