#!/usr/bin/env just --justfile

set unstable
docker_output := 'build'
programm_name := 'projektor'

# help
[group('common')]
default: help

# List all the just commands
[group('common')]
help:
    @just --list

[group('rust')]
build-static:
    cargo build --release --target=x86_64-unknown-linux-musl

[group('rust')]
build-dockerized:
    rm -r "{{justfile_directory()}}/{{docker_output}}" || true
    docker build --target artifact --output type=local,dest={{docker_output}} .
