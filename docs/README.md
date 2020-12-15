# Warning, legal related.
Since you are **not** supposed to do this. It means that this is of course **illegal** use this at your own risk. I will not be responsible for anything that happens to you due to your use of this application, and will not be held accountable. License found [here](https://github.com/YJH16120/UMA/blob/main/docs/license)

# UMA (Untitled Media App)
A video demonstration can be found [here](https://youtu.be/Pi5b0CzH4EM)
The purpose of UMA is to go around those pesky spotify ad barriers, that will disrupt the user experience. 
Documentation located [here](https://github.com/YJH16120/UMA/blob/main/docs.md)

# Download instructions
## Prerequisites
1. Have python installed:
#### Windows installation 
If you don't have python download it [here](https://www.python.org/ftp/python/3.9.0/python-3.9.0-amd64.exe). A note is that once you click this link, it will __immediately__ download python.exe. This is only for __windows__.

#### Ubuntu/Debian installation
You do not need to install anything, if you're already on a Linux distro since python comes installed by default, but to make sure it is python 3.5 and above, use the command `python3 --version`. If it is
greater than version 3.5 go ahead and proceed to the next step. If not go to the terminal and write the following into the terminal
```
sudo apt update
sudo apt -y upgrade
```
Then check the version of python again with `python3 --version`

2. Have Firefox installed:
Install the firefox browser if you haven't already, as this program uses firefox.

3. Have ghostery.xpi:
After installing the ghostery browser extension from the firefox web store, go to the search bar in firefox and type in `about:support` and find the row that reads
"Profile folder", and click the button "open folder". Then find the extensions folder, and find the file `firefox@ghostery.com.xpi` and rename it to
`ghostery.xpi` keep the extension, unchanged.

4. Move `ghostery.xpi` to `src/` then all you need to do is run `py GUI.py` (For Ubuntu/Debian it is `python3 GUI.py`).

#### Which OS' are supported?
Currently the only supported OS's are the Windows OS and Ubuntu/Debian. Since these are the OS' thay I've tested my program on. 

# Why use UMA?
UMA has the ability to download any media you are currenly playing. Such as a song you really like. Just enter "download" in the terminal
and it will download the desired file into a directory called "media".

### Known limitations
1. Unable to display the following content available. Similar to YouTube, where there is a column with all other videos you can possible choose from. (Won't fix)

3. Unable to skip straight to next video, once it has ended (Won't fix)

4. Unable to skip straight to next video, on command. (Won't fix)
