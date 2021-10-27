#!/bin/sh

deps() {
	echo "checking dependencies..."
	which cargo > /dev/null
	if [ ! "$?" = 0 ]; then
		echo "[ERROR] \`cargo\` couldn\'t be found in $PATH"
		exit 1
	fi
}

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
	deps
	args "$@"
	echo "generating version from Cargo.toml..."
	echo "$(grep -v 'const VERSION_STR' src/args.rs)
const VERSION_STR: &str = \"vct: v$(grep 'version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')\";" > src/args.rs
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
	echo "generating makefile..."
	echo "$PREFIX
$MAKEFILE" > Makefile
	echo "done!"
}

main "$@"
