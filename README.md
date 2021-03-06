# VCT
## a vocabulary trainer for the terminal
[![pipeline status](https://gitlab.sokoll.com/moritz/vct/badges/main/pipeline.svg)](https://gitlab.sokoll.com/moritz/vct/-/commits/main)

`vct` is a simple utility to learn vocabulary. It allows you to manage your vocabulary in categories and more.
It will question you about the meanings of a vocabulary and optionally additional information (like gender if needed).

## Installation
### Stable (recommended)
Stable is the preferred version to install. It may not have the newest and greatest but it will run smoothly
#### AUR
`vct` is in the AUR as `vct`. You can just install it that way
#### source
1. clone the repository
2. run `./configure.sh -p <prefix>` to set the prefix to install to
3. run `make install`
### Nightly
Nightly is a kind of stable version but still in development. You will probably encounter bugs
but it is usable. Nightly also receives updates more frequently than stable and you will get features earlier
#### source
1. clone the repository
2. run `./configure.sh -p <prefix>` to set the prefix to install to (preferably `/usr/local`)
3. run `make install`

## Usage
### Learning
To learn existing vocabulary use the `-l` or `--lang` option followed by whatever language you want to learn.
It will search for that language in a path set in the config file and the default option in `$XDG_CONFIG_HOME/vct/dicts`.
Then it will ask you all the vocabulary in a random order and you type in its meaning.
If a vocabulary has multiple meanings it will ask multiple times.
When you're finished it will tell you how many you had right and it will show a small bar with the percentage.

### Creating a set of vocabulary
To create a set of vocabulary use the `-d` or `--dict` flag followed by:
1. the language
2. the vocabulary
3. the meanings as a comma separated list
if the language doesn't already exist it will be created.

### Querying existing vocabulary
`vct` allows for querying existing vocabulary using the `-q` or `--query` option
followed by the pattern to search for. It will search in meaning and
name. Currently you cannot filter by language.

## Configuration
The configuration file is in `$XDG_CONFIG_HOME/vct/config.toml` (if `$XDG_CONFIG_HOME` doesn't exist it will be in `~/.config/vct/config.toml`)
and currently has only two fields:
```toml
dict = "dicts"
vocab = "one"
additionals = true
clearlines = true
database = false # database is not recommended currently since there is a lack of support for writing and migrating from dict files
dbpath = "vocab.db"
```
- `dicts`: a list of strings/paths where dictionaries are. If the path doesn't start with a slash
  it gets automatically put in the config directory
- `vocab`: can be `one` or `all`. Defines how many meanings get learned per vocab (can be overwritten with `-V` or `--vocab`)
- `additionals`: can be `true` or `false`. Tells `vct` whether or not to ask you for additional information about the vocabulary (can be overwritten with `--adds` and `--noadds`)
- `clearlines`: can be `true` or `false`. Tells `vct` whether or not to clear unused lines (can be overwritten with `--clear` and `--noclear`)

## Development
It is currently only developed by me.
If you encounter any bugs report them [here](https://github.com/ULUdev/vct/issues/new).
If you want to add a new feature or help development otherwise feel free to make a pull request on GitHub.
