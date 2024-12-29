#!/bin/sh

cargo +nightly miri test -Znext-lockfile-bump
