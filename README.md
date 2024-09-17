# touch-for-windows

A lightweight Windows implementation of Unix's `touch` command written in Rust. This package is built for Windows specifically, and haven't been tested on other systems.

## Install

For users: `cargo install touch-for-windows`

For developers:

1. `git clone https://www.github.com/manuelinfosec/touch-for-windows.git`
2. `cd touch-for-windows`
3. `cargo install --path .`

## Usage

`touch [option] <filename>`

### Options

| option            | functionality                                                                                                                                         |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `-a`              | change only the access time of a file                                                                                                                 |
| `-d`, `--date`    | specifies the date to use when changing access and/or modification/write time |                                           |
| `-m`              | change only the modification/write time of a file                                                                                                     |
| `-a`              | change only the access time of a file                                                                                                     |
| `--help`          | display help page                                                                                                                                     |


## Examples

Touch a file: `touch my-file.txt`

Change only the access time: `touch -a my-file.txt`

Change only the modification/write time: `touch -m my-file.txt`

Change both access time and modification/write time: `touch -am my-file.txt` (this is the same behavior as simply `touch my-file.txt`)

Toggle hidden attribute of a file: `touch -h my-file.txt` (hides/unhides a file if it exists or creates a hidden file if it does not)

Touch multiple files: `touch my-file.txt package.json another-file.js`

Manually specify the date to use: `touch -d="8/25/2054 6:35:56 AM" my-file.txt`

Using multiple flags together: `touch -am -d="5/4/2020" my-file.txt`