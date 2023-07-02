#!/usr/bin/env bash

sqlx database reset --source ./migrations/test
cargo run
