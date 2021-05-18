# Untitled Media Player (UMA)
![](Images/UMA.png)  

---
This is just a personal project I made for fun, I refactored it to Rust instead, because I believe that it's easier to make this with Rust than with something like Python.

This is a terminal music player, with slight TUI elements. It plays music from your local machine. Video demonstartion available on [youtube](https://www.youtube.com/watch?v=teCQJ0yYcQs&t=73s).

# Installation
### Downloading through the source code
**pre-requisites**  
1. Have the Rust programming language installed.

Simply execute these commands:
```
git clone 'https://github.com/YJH16120/UMA/'
cd UMA
cargo build --release
mv target/release/uma `path`
```
Where `path` is your path, to find out which folders are in your path simply perform `echo $PATH`.

# Usage
### Select a directory to look for audio files
UMA needs to know which directory to look for songs it. Simply specify either the absolute or relative path.
As long as it's valid, there will be no issues.

You have the ability to change where UMA looks for files. There are two methods to do this:
1. Manually by editing the config directly,
2. pressing `shift+c` when in UMA.

### Commands
UMA features basic commands, and they're inspired by vim motions.

### Volume control
K to increase volume, J to decrease. Volume increments or decrements by 0.05.

### Exiting the player
Press ESC to exit. 
- This only works when audio is actually playing, if you want to quit at anytime abruptly just break with ctrl+c or cmd+c

### Pausing/Playing
Just press P. UMA will know how to alternate.

# Note
It is unsure whether or not this application is suppported on Windows. If it isn't supported, don't count on me 
adding support for it on Windows. 
