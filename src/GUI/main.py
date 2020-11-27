# Built in packages
from tkinter import *
import sys, os

# Custom packages
from player import *

class Application(Frame, Player):
    def __init__(self, master=None):
        Frame.__init__(self, master=None)
        Player.__init__(self)
        self.master = master

        self.Window()

    def Window(self):
        MainTitle = Label(self.master, text="Welcome", font=("Linux Libertine G", 20))
        MainTitle.place(x=150 , y=0)

        QuitBtn = Button(self.master, text="Quit", command=self.AbsoluteExit, padx=10, fg="red", font=20)
        QuitBtn.place(x=310, y=0)

        self.EnterSongName = Label(self.master, text="Enter song name: ")
        self.EnterSongName.place(x=50, y=50)

        self.SongEntry = Entry(self.master)
        self.SongEntry.place(x=150, y=50)

        FindBtn = Button(self.master, text="Find song", command=self.Search)
        FindBtn.place(x=280 , y=50)

    def Search(self):
        video = self.SongEntry.get()
        Player.Play(self, video)
        self.SelectVideo()

    def SelectVideo(self):
        ReferenceByNumber = Label(self.master, text="Reference by number: ")
        ReferenceByNumber.place(x=50, y=90)

        RefByNum = Entry(self.master, borderwidth=3)
        RefByNum.place(x=170, y=90)

        ConfSelection = Button(self.master, text="Press to confirm", command=lambda: self.GetText(RefByNum))
        ConfSelection.place(x=300, y=90)

    def GetText(self, e):
        selector = e.get()
        Player.SelectVideo(self, selector)
        self.CreateControlButton()

    def CreateControlButton(self):
        PlayBtn = Button(self.master, text="Play/Pause", fg="green", command=lambda: self.PlayerControl("p"))
        PlayBtn.place(x=50, y=120)

        ReplayBtn = Button(self.master, text="Replay", fg="green", command=lambda: self.PlayerControl("r"))
        ReplayBtn.place(x=140, y=120)

        PlayDownload = Button(self.master, text="Download", fg="green", command=lambda: self.PlayerControl("dl"))
        PlayDownload.place(x=190, y=120)

        self.check = BooleanVar()
        mp3 = Checkbutton(self.master, text="mp3", variable=self.check, onvalue=True, offvalue=False)
        mp3.place(x=280, y=120)

    def PlayerControl(self, cmd):
        check = self.check.get()
        Player.PlayerControl(self, command=cmd, mp3=check)

    def AbsoluteExit(self):
        Player.AbsoluteExit(self)
        sys.exit()

try:
    master = Tk()
    master.geometry("400x400")

    App = Application(master)
    master.mainloop()

except Exception as e:
    print(e)
    sys.exit()
