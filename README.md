# VCT
## a vocabulary trainer for the terminal

## Installation
### AUR
`vct` is in the AUR as `vct`. You can just install it that way
### source
1. clone the repository
2. run `./configure.sh -p <prefix>` to set the prefix to install to
3. run `make install`

## Usage
### Learning
To learn existing vocabulary use the `-l` or `--lang` option followed by whatever language you want to learn.
It will search for that language in a path set in the config file and the default option in `$XDG_CONFIG_HOME/vct/dicts`.
Then it will ask you all the vocabulary in a random order and you type in its meaning.
If a vocabulary has multiple meanings it will ask multiple times.
When you're finished it will tell you how many you had right and it will show a small bar with the percentage.

### Creating a set of vocabulary
To create a set of vocabulary use the `-D` or `--dict` flag followed by:
1. the language
2. the vocabulary
3. the meanings as a comma separated list
if the language doesn't already exist it will be created.

## Configuration
The configuration file currently only allows you to overwrite the default set of directories
to search for dictionaries.
