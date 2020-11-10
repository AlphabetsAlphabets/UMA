# Built in packages
from tkinter import *
import sys, os

# Custom packages

class Application(Frame):
    count = 0
    def __init__(self, master=None):
        Frame.__init__(self, master=None)
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
        self.video = self.SongEntry.get()

        self.P = Player(self.video)
        self.P.Play()

        self.SelectVideo()

    def SelectVideo(self):
        ReferenceByNumber = Label(self.master, text="Reference by number: ")
        ReferenceByNumber.place(x=50, y=90)

        self.RefByNum = Entry(self.master, borderwidth=3)
        self.RefByNum.place(x=170, y=90)

        ConfSelection = Button(self.master, text="Press to confirm", command=self.GetText)
        ConfSelection.place(x=300, y=90)

    def GetText(self):
        self.text = self.RefByNum.get()
        self.WriteToCommand()
        self.CreateControlButton()

    def CreateControlButton(self):
        self.PlayBtn = Button(self.master, text="Pause", fg="green", command=lambda: self.WriteToCommand("p"))
        self.PlayBtn.place(x=50, y=120)

        self.PlayReplay = Button(self.master, text="Replay", fg="green", command=lambda: self.WriteToCommand("r"))
        self.PlayReplay.place(x=100, y=120)

        self.PlayDownload = Button(self.master, text="Download", fg="green", command=lambda: self.WriteToCommand("dl"))
        self.PlayDownload.place(x=160, y=120)

    def AbsoluteExit(self):
        self.P.AbsoluteExit()
        sys.exit()

try:
    master = Tk()
    master.geometry("400x400")

    App = Application(master)
    master.mainloop()

except Exception as e:
    print(e)
    App.AbsoluteExit()
    sys.exit()
