#!/usr/bin/env bash
# Run `zebar open bar --args ...` for every monitor.
zebar monitors --print0 | xargs -0 -P 99 -I % sh -c 'zebar open bar --args %'
