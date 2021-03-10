#!/usr/bin/env bash

echo "Remember to update the compatibility versions in lib.rs"

bindgen src/vendor/evdi/library/evdi_lib.h -o src/bindings.rs -- \
  --include stdint.h

bindgen src/c_wrapper/wrapper.h -o src/wrapper_bindings.rs
