#/bin/bash

abort() {
    echo "$@" >&2
    exit 1
}

autoreconf --install --force --verbose || abort "autoreconf failed"
echo "Please run ./configure, make and make install"
