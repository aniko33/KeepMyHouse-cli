#!/bin/bash

cargo build
./target/debug/kmh-cli $@
