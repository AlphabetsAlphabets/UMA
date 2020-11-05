from time import sleep
import sys

from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.firefox.options import Options
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

from media_downloader import Media
from bs4 import BeautifulSoup as BS

class Player:
    ghostery = "D:\\Coding\\python\\web\\WebPlayer\\ghostery.xpi"
    opts = Options()
    opts.headless = True
    driver = webdriver.Firefox(options=opts)
    driver.install_addon(ghostery, temporary=False)

    self.AutoPlay = False

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

    def init(self, video):
        video = input("Channel/Video: ")

        if " " in video:
            term = video.split(" ")
            term = "+".join(term)
            self.url = f"https://www.youtube.com/results?search_query={term}"

        else:
            self.url = f"https://www.youtube.com/results?search_query={term}"

    def Play(self):
        self.driver.get(self.url)
        sleep(0.5)

        source = self.driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")

        videoClass = "yt-simple-endpoint style-scope ytd-video-renderer"

        videos = soup.find_all('a', class_=videoClass)

        print("=="*20)
        for index, video in enumerate(videos, start=1):
           print(f"{index}. {video['title']}")

        select = int(input("Reference video by number: "))
        print("=="*20)

        vidLink = videos[select - 1]['href']
        self.link = f"https://www.youtube.com{vidLink}"
        self.driver.get(self.link)
        sleep(1)
        try:
            screen = self.driver.find_element(By.XPATH, "/html/body/ytd-app/div/ytd-page-manager/ytd-watch-flexy/div[4]/div[1]/div/div[1]/div/div/div/ytd-player/div/div")
            screen.click()

        except Exception as e:
            print(e)
            self.Play()

    def NextVideos(self):
        source = self.driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")

        try:
            UpNext = soup.find_all("div", class_="yt-simple-endpoint style-scope ytd-playlist-panel-video-renderer")
            print("=="*20)
            for index, item in enumerate(UpNext):
                print(f"{index}. {item}")

            print("=="*20)

            select = input("Reference video by number: ")
            vidLink = UpNext[select - 1]['href']
            self.link = f"https://www.youtube.com{vidLink}"
            self.Play()
            self.side = True

        except:
            UpNext = soup.find_all("div", class_="yt-simple-endpoint inline-block style-scope ytd-thumbnail")
            for index, item in enumerate(UpNext):
                print(f"{index}. {item}")

            print("=="*20)

            select = input("Reference video by number: ")
            vidLink = UpNext[select - 1]['href']
            self.link = f"https://www.youtube.com{vidLink}"

        self.PlayerControl()

    def SkipToNextVideo(self):
        source = self.driver.execute_script("return document.documentElement.outerHTML")
        soup = BS(source, "lxml")
        duration = soup.find("span", class_="ytp-time-duration")
        duration = duration.split(":")
        duration = ".".join(duration)
        duration = float(duration) * 60 + 0.5

        try:
            autoplay = "ytp-upnext-autoplay-icon"
            element = WebDriverWait(self.driver, duration+0.5)
            element.until(EC.presence_of_element_located((By.CLASS_NAME, autoplay)))
            self.PlayerControl()
        except:
            pass

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
        command = input("Input commands: ").lower()

        if not self.autoplay:
            if command == "p": # Play, Pause mechanism
                screen_path = "/html/body/ytd-app/div/ytd-page-manager/ytd-watch-flexy/div[4]/div[1]/div/div[1]/div/div/div/ytd-player/div/div"
                screen = self.driver.find_element(By.XPATH, screen_path)
                screen.click()

            elif command == "av":
                self.Play()

            elif command == "r":
                self.Replay()

            elif command == "q":
                self.driver.quit()
                sys.exit()

            elif command == "dl":
                check = input("Do you want an .mp3 of this? y/n: ").lower()
                if check == "y":
                    check = True
                else:
                    check = False

                M = Media(video=self.link, mp3=check)
                M.download()

            elif command == "nv":
                self.NextVideos()
        else:
            print("You are able to skip to the next video.")
            select = input("Would you like to? y/n: ").lower()
            if select == 'y':
                pass
