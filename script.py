from time import sleep

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.chrome.options import Options

from bs4 import BeautifulSoup as BS

class Player:
    ghostery = "D:\\Coding\\python\\web\\WebPlayer\\ghostery.crx"
    opts = Options()
    opts.add_extension(ghostery)
    # opts.add_argument("--headless")
    driver = webdriver.Chrome(options=opts)

    def init(self):
        pass

    def Play(self):
        video = input("Channel/Video: ")

        if " " in video:
            term = video.split(" ")
            term = "+".join(term)
            self.url = f"https://www.youtube.com/results?search_query={video}"

        else:
            self.url = f"https://www.youtube.com/results?search_query={video}"

        sleep(0.8)
        self.driver.get(self.url)
        sleep(0.5)

        source = self.driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")

        videoClass = "yt-simple-endpoint style-scope ytd-video-renderer"

        videos = soup.find_all('a', class_=videoClass)

        for index, video in enumerate(videos, start=1):
           print(f"{index}. {video['title']}")

        select = input("Reference video by number: ")
        index = int(index)
        vidLink = videos[index - 1]['href']

        self.driver.get(f"https://www.youtube.com/{vidLink}")
        self.replay_link = f"https://www.youtube.com/{vidLink}"

    def PlayerControlGuide(self):
        print("To use the player control feature, go ahead and use mnemonics. Currently available features are: ")
        print("Pause, Play, and search for another video entirely.")
        print("Use mnemonics, P for pause, P again to unpause, and AV for another video. It isn't case sensitive.")
        print("So lower case p will have the same effect as uppercase p.")
        print("--"*20)

        print("Full list available at: ")

    # Player control code begins here.

    def Replay(self):
        self.driver.get(replay_link)

    def PlayerControl(self):
        command = input("Input commands: ").lower()

        if command == "p": # Play, Pause mechanism
            screen = self.driver.find_element(By.XPATH, "/html/body/ytd-app/div/ytd-page-manager/ytd-watch-flexy/div[4]/div[1]/div/div[1]/div/div/div/ytd-player/div/div")
            screen.click()

        elif command == "av":
            self.Play()

        elif command == "r":
            self.replay()

        elif command == "q":
            self.driver.quit()
            sys.exit()
