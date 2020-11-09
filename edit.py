# Built in packages
import sys, os
from time import sleep
from datetime import datetime

# Selenium

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.action_chains import ActionChains as AC

from bs4 import BeautifulSoup as BS

# Custom packages
from media_downloader import Media

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

    def __init__(self, video):
        self.video = video

    def Play(self):
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
        self.Output()

    def Output(self):
        print("=="*20)
        for index, video in enumerate(self.videos, start=1):
           print(f"{index}. {video['title']}")

        print("=="*20)

    def ReadFromCommand(self):
        with open("commands.txt", 'r') as f:
            select = f.readlines()

        try:
            PlayerCheck = select[1]
            if PlayerCheck == "PlayerControl":
                pass
        except IndexError:
            try:
                self.vidLink = self.videos[int(select[0].strip("\n")) - 1]['href']

            except ValueError:
                print(select[0].strip("\n"), "is not a number.")
                self.driver.quit()
                sys.exit()

        self.link = f"https://www.youtube.com{self.vidLink}"
        self.driver.get(self.link)

        sleep(0.7)
        guard = 0
        while guard <= 80:
            try:
                action = AC(self.driver)
                action.send_keys("k")
                action.perform()
                break

            except Exception as e:
                if guard >= 50 and guard <= 70:
                    screen = self.driver.find_element(By.XPATH, "/html/body/ytd-app/div/ytd-page-manager/ytd-watch-flexy/div[4]/div[1]/div/div[1]/div/div/div/ytd-player/div/div")
                    screen.click()
                    break
                else:
                    guard += 1

    # Player control code begins here.
    def Replay(self):
        self.driver.get(self.link)
        while True:
            try:
                screen_path = "/html/body/ytd-app/div/ytd-page-manager/ytd-watch-flexy/div[4]/div[1]/div/div[1]/div/div/div/ytd-player/div/div"
                screen = self.driver.find_element(By.XPATH, screen_path)
                screen.click()
                break
            except:
                continue

    def PlayerControl(self):
        with open("commands.txt", "r") as f:
            command = f.readlines()

        command = command[0]

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
            check = input("Do you want an .mp3 of this? y/n: ").lower()
            if check == "y":
                check = True
            else:
                check = False

            M = Media(video=self.link, mp3=check)
            M.download()

    def AbsoluteExit(self):
        self.driver.close()
        sleep(0.1)
        sys.exit()