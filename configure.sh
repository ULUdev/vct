#!/bin/sh

args() {
	local opts
	while getopts "p:" opts; do
		case $opts in
			p)
				PREFIX="$OPTARG"
				;;
		esac
	done
}

main() {
	local MAKEFILE PREFIX
	args "$@"
	PREFIX="PREFIX=$PREFIX
"
	MAKEFILE='all: release

release: target/release/vct

target/release/vct:
	cargo build --release

install: release
	mv target/release/vct $(PREFIX)/bin/

clean:
	cargo clean'
	echo "$PREFIX
$MAKEFILE" > Makefile
}

main "$@"
