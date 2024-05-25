set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

install:
    cargo install --path .
    
uninstall:
    cargo uninstall mabel

lint:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery