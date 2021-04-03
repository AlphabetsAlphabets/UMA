# Untitled Media Player (UMA)
This is just a personal project I made for fun, I refactored it to Rust instead, because I believe that it's easier to make this with Rust than with something like Python.

This is a terminal music player, with slight TUI elements. It plays music from your local machine.

# Installation
There are two methods to install:
1. UMA either through the source code,
2. The releases page.

### Downloading through the source code
Simply execute these commands:
```
git clone 'https://github.com/YJH16120/UMA/'
cd UMA
cargo build --release
mv target/release/uma `path`
```
Where `path` is your path, to find out which folders are in your path simply perform `echo $PATH`.

### Downloading through the releases
Simpley go to this page, download the file some where on your system then run the following commands:
```
mv path/to/uma/binary `path`
```


# Usage
### Select a directory to look for audio files
UMA needs to know which directory to look for songs it. Simply specify either the absolute or relative path.
As long as it's valid, there will be no issues.

### Commands
UMA features basic commands, and they're inspired by vim motions.

### Volume control
J to increase volume, K to decrease. Volume increments or decrements by 0.25.

### Exiting the player
Press ESC to exit. 
- This only works when audio is actually playing, if you want to quit at anytime abruptly just break with ctrl+c or cmd+c

### Pausing/Playing
Just press P. It'll know how to alternate.

# Note
It is unsure whether or not this application is suppported on Windows. If there is an issue, feel free to make an issue.
