#!/usr/bin/env bash

zebar monitors --print0 | xargs -0 -P 99 -I % sh -c 'zebar open bar --args %'
