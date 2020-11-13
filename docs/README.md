# Warning, legal related.
Since you are **not** supposed to do this. It means that this is of course **illegal** use this at your own risk. I will not be responsible for anything that happens to you due to your use of this application, and will not be held accountable. License found [here](https://github.com/YJH16120/UMA/blob/main/docs/license)

# UMA (Untitled Media App)
The purpose of UMA is to go around those pesky spotify ad barriers, that will disrupt the user experience. Everything will be done through the terminal as I am __terrible__ at graphic design. However, I am currently developing on a GUI that can work with UMA's script.

I will accept any and all help, so feel free to dm me at discord. My tag is: 叶家煌(Jia Hong)#8464.

Documentation located [here](https://github.com/YJH16120/UMA/blob/main/docs.md)

# Using UMA

## Prerequisites
1. Have python installed:
If you don't have python download it [here](https://www.python.org/ftp/python/3.9.0/python-3.9.0-amd64.exe). A note is that once you click this link, it will __immediately__ download python.exe.

2. Have Firefox installed:
Install the firefox browser if you haven't already, as this program uses firefox.

3. Have ghostery.xpi:
Go to the search bar in firefox and type in `about:support` and find the row that reads "Profile folder", and click the button "open folder". Then find the extensions folder,
and find the file `firefox@ghostery.com.xpi` and rename it to `ghostery.xpi` keep the extension, unchanged.

### Instructions for the CLI version
Navigate to the `src/CLI` folder, and make sure to `pip install -r requirements.txt` before hand. Move the `ghostery.xpi` file to `src/CLI`. Finally, run `script.py` with python: `py script.py` for
UNIX/Linux run `python3 script.py`

### Instructions for the GUI version
Navigate to the `src/GUI/` folder. Make sure to `pip install -r requirements.txt` before you do this, or it won't work. And that the `ghostery.xpi` file is present in the `src/GUI` directory.
Then finally run the file `main.py` with `py main.py` if you are on UNIX/Linux run `python3 main.py` instead.

### CLI Demonstration
Link to video [here](https://youtu.be/E6Tb2xtnc3o)

### GUI Demonstration
Link to video [here](https://youtu.be/Pi5b0CzH4EM)

# Why use UMA?
UMA has the ability to download any media you are currenly playing. Such as a song you really like. Just enter "download" in the terminal
and it will download the desired file into a directory called "media".

**This has only been tested on windows.**

### Known limitations
1. Unable to display the following content available. Similar to YouTube, where there is a column with all other videos you can possible choose from. (Won't fix)

3. Unable to skip straight to next video, once it has ended (Won't fix)

4. Unable to skip straight to next video, on command. (Won't fix)
