from tkinter import *
from script import Player
import sys

class Application(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master=None)
        self.master = master
        self.text = "Nothing"

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

        FindBtn = Button(self.master, text="Find song.", command=self.Search)
        FindBtn.place(x=280 , y=50)

    def Search(self):
        self.video = self.SongEntry.get()

        ReferenceByNumber = Label(self.master, text="Reference by number: ")
        ReferenceByNumber.place(x=50, y=90)

    def SelectVideo(self):
        ConfSelection = Button(self.master, text="Press to confirm", command=lambda: self.GetText(RefByNum))
        ConfSelection.place(x=300, y=90)

        RefByNum = Entry(self.master, borderwidth=3)
        RefByNum.insert(0, "Reference by number")
        RefByNum.place(x=170, y=90)

    def StartMedia(self):
        P = Player(self.video)
        P.Play()

    def GetText(self, e):
        self.text = e.get()
        self.StartMedia()
        self.WriteToCommand()

    def WriteToCommand(self):
        with open('commands.txt', 'w') as f:
            f.write(self.text)

    def AbsoluteExit(self):
        sys.exit()

try:
    master = Tk()
    master.geometry("400x400")

    App = Application(master)
    master.mainloop()

except:
    sys.exit()