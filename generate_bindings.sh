#!/usr/bin/env bash

bindgen src/vendor/evdi/library/evdi_lib.h -o src/bindings.rs -- \
  --include stdint.h
