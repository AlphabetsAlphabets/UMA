# Warning, legal related.
Since you are **not** supposed to do this. It means that this is of course **illegal** use this at your own risk. I will not be responsible for anything that happens to you due to your use of this application, and will not be held accountable. License found [here](https://github.com/YJH16120/UMA/blob/main/docs/license)

# UMA (Untitled Media App)
The purpose of UMA is to go around those pesky spotify ad barriers, that will disrupt the user experience. 
Users have the choice between operating the Command line version [(CLI)](https://github.com/YJH16120/UMA/tree/main/src/CLI) or the User interface version [(GUI)](https://github.com/YJH16120/UMA/tree/main/src/GUI). 

Documentation located [here](https://github.com/YJH16120/UMA/blob/main/docs.md)

# Using UMA

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

### Instructions for the CLI version
#### CLI Demonstration
Link to video [here](https://youtu.be/E6Tb2xtnc3o)

#### Windows installation
Navigate to the `src/CLI` folder, and make sure to `pip install -r requirements.txt` before hand. Move the `ghostery.xpi` file to `src/CLI`. Finally, run `use.py` with python: `py use.py`

#### Ubuntu/Debian installation
Navigate to the `src/CLI` folder, and make sure to `pip install -r requirements.txt`, then move `ghostery.xpi` file to `src/CLI`. Then install tkinter with `sudo apt-get python3-tk` (Which is the GUI framework)
Next run `python3 use.py`

### Instructions for the GUI version
#### GUI Demonstration
Link to video [here](https://youtu.be/Pi5b0CzH4EM)

#### Windows installation
Navigate to the `src/CLI` folder, and make sure to `pip install -r requirements.txt` before hand. Move the `ghostery.xpi` file to `src/CLI`. Finally, run `GUI.py` with python: `py GUI.py`

#### Ubuntu/Debian installation
Navigate to the `src/CLI` folder, and make sure to `pip install -r requirements.txt`, then move `ghostery.xpi` file to `src/CLI`. Then install tkinter with `sudo apt-get python3-tk` (Which is the GUI framework)
Next run `python3 GUI.py`

#### Which OS' are supported?
Currently the only supported OS's are the Windows OS and Ubuntu/Debian. Since these are the OS' thay I've tested my program on. 

# Why use UMA?
UMA has the ability to download any media you are currenly playing. Such as a song you really like. Just enter "download" in the terminal
and it will download the desired file into a directory called "media".


### Known limitations
1. Unable to display the following content available. Similar to YouTube, where there is a column with all other videos you can possible choose from. (Won't fix)

3. Unable to skip straight to next video, once it has ended (Won't fix)

4. Unable to skip straight to next video, on command. (Won't fix)
