#!/bin/sh

deps() {
	echo "checking dependencies..."
	which cargo > /dev/null
	if [ ! "$?" = 0 ]; then
		echo "[ERROR] \`cargo\` couldn't be found in \$PATH"
		exit 1
	fi
	which git > /dev/null
	if [ ! "$?" = 0 ]; then
		echo "[ERROR] \`git\` couldn't be fount in \$PATH"
		exit 1
	fi
}

args() {
	local opts help
	help="
Usage:
  ./configure.sh [OPTIONS]
Options:
  -h: print this help and exit
  -p <prefix>: set the prefix to install to
  -g: prepare files for git repo and exit (used for developer purposes only)
"
	while getopts "hgp:" opts; do
		case $opts in
			h)
				echo "$help" > /dev/stderr
				exit 0
				;;
			p)
				PREFIX="$OPTARG"
				;;
			g)
				echo "preparing for git..."
				echo "$(grep -v -e 'const VERSION_STR' -e 'commit: .*;$' src/args.rs)
const VERSION_STR: &str = \"vct: v$(grep 'version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')\";" > src/args.rs
				echo "done!"
				exit 0
				;;
		esac
	done
}

main() {
	local MAKEFILE PREFIX version
	deps
	args "$@"
	version=$(grep 'version' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
	if [[ $version == *"nightly"* ]]; then
		version="$version
commit: $(git --no-pager log -n 1 --pretty=format:"%H")"
	fi
	echo "generating version from Cargo.toml..."
	echo "$(grep -v -e 'const VERSION_STR' -e 'commit: .*;$' src/args.rs)
const VERSION_STR: &str = \"vct: v$version\";" > src/args.rs
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
