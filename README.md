# touch-for-windows

A lightweight Windows implementation of Unix's `touch` command written in Rust. This package is built for Windows specifically, and haven't been tested on other systems.

## Install

For users: `cargo install touch-for-windows`

For nerds:

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

- **Create a file (or update its timestamps if it exists)**:  
  `touch my-file.txt`

- **Update only the access time**:  
  `touch -a my-file.txt`

- **Update only the modification (write) time**:  
  `touch -m my-file.txt`

- **Update both access and modification times (default behavior)**:  
  `touch -am my-file.txt` (same as `touch my-file.txt`)

- **Touch multiple files at once**:  
  `touch my-file.txt package.json another-file.js`

- **Specify a custom date and time**:  
  `touch -d "8/25/2054 6:35:56 AM" my-file.txt`

- **Use multiple flags (update both times with a custom date)**:  
  `touch -am -d "5/4/2020" my-file.txt`