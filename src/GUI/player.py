# Built in packages
import sys, os
from time import sleep

# Selenium
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.action_chains import ActionChains as AC


# Custom packages
from media import *

class Player:
    ghostery = os.getcwd() + "\\ghostery.xpi"
    opts = Options()
    opts.headless = True
    driver = webdriver.Firefox(options=opts)
    driver.install_addon(ghostery, temporary=False)

    sleep(0.5)
    print("Starting session.")

    while True:
        try:
            driver.switch_to_window(driver.window_handles[1])
            driver.close()
            driver.switch_to_window(driver.window_handles[0])
            break

        except:
            continue

    def __init__(self):
        pass

    def Play(self, video):
        self.video = video
        if " " in self.video:
            self.term = self.video.split(" ")
            self.term = "+".join(self.term)
            self.url = f"https://www.youtube.com/results?search_query={self.term}"

        else:
            self.url = f"https://www.youtube.com/results?search_query={self.video}"

        self.driver.get(self.url)
        sleep(0.5)

        source = self.driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")

        videoClass = "yt-simple-endpoint style-scope ytd-video-renderer"

        self.videos = soup.find_all('a', class_=videoClass)

        print("=="*20)
        for index, video in enumerate(self.videos, start=1):
           print(f"{index}. {video['title']}")

        print("=="*20)

    def SelectVideo(self, selector):
        vidlink = self.videos[int(selector) - 1]['href']
        self.vidlink = f"https://www.youtube.com{vidlink}"
        self.driver.get(self.vidlink)

        sleep(0.1)
        action = AC(self.driver)
        action.send_keys("k")
        action.perform()

    # Player control code begins here.
    def Replay(self):
        self.driver.get(self.vidlink)
        sleep(0.1)
        action = AC(self.driver)
        action.send_keys("k")
        action.perform()

    def PlayerControl(self, command, mp3=False):
        if mp3 is False:
            if command == "p": # Play, Pause mechanism
                action = AC(self.driver)
                action.send_keys("k")
                action.perform()

            elif command == "av":
                self.Play()

            elif command == "r":
                self.Replay()

            elif command == "q":
                self.AbsoluteExit()

            elif command == "dl":
                M = Media(video=self.vidlink, mp3=mp3)
                M.download()

        else:
            if command == "p": # Play, Pause mechanism
                action = AC(self.driver)
                action.send_keys("k")
                action.perform()

            elif command == "av":
                self.Play()

            elif command == "r":
                self.Replay()

            elif command == "q":
                self.AbsoluteExit()

            elif command == "dl":
                M = Media(video=self.vidlink, mp3=mp3)
                M.download()


    def AbsoluteExit(self):
        self.driver.close()
        sleep(0.1)
        sys.exit()
