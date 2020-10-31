from time import sleep

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.chrome.options import Options

from bs4 import BeautifulSoup as BS

class Player:
    ghostery = "D:\\Coding\\python\\pet\\downloader\\ghostery.crx"
    opts = Options()
    opts.add_extension(ghostery)
    self.driver = webdriver.Chrome(options=opts)

    def __init__(self, video):
        if " " in video:
            term = video.split(" ")
            term = "+".join(term)
            self.url = f"https://www.youtube.com/results?search_query={video}"

        else:
            self.url = f"https://www.youtube.com/results?search_query={video}"

    def main(self):
        driver.get(self.url)

        source = driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")

        videoClass = "yt-simple-endpoint style-scope ytd-video-renderer"

        videos = soup.find_all('a', class_=videoClass)

        for index, video in enumerate(videos, start=1):
           print(f"{index}. {video['title']}")

        select = input("Reference video by number: ")
        vidLink = videos[int(index) - 1]['href']

        self.replay_link = f"https://www.youtube.com{vidLink}"

    def PlayerControlGuide(self):
        print("To use the player control feature, go ahead and use mnemonics. Currently available features are: ")
        print("Pause, Play, and search for another video entirely.")
        print("Use mnemonics, P for pause, P again to unpause, and AV for another video. It isn't case sensitive.")
        print("So lower case p will have the same effect as uppercase p.")
        print("--"*20)

        query = input("A full list of features is available would you like to view them?")
        if query == 'yes':
            pass # Insert display code here
        else:
            pass # Otherwise feature

    # Player control code begins here.

    def replay(self):
        self.driver.get(self.replay_link)

    def PlayerControl(self):
        command = input("Input commands: ").lower()

        if command == "p": # Play, Pause mechanism
            try:
                playButton = driver.find_element(By.TITLE, "Pause (k)")
                playButton.click()
            except:
                pauseButton = driver.find_element(By.TITLE, "Play (k)")
                pauseButton.click()

        elif command == "av":
            self.main()

        elif command == "r":
            self.replay()
