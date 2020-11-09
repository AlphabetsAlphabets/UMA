from tkinter import *
from edit import Player
import sys

class Application(Frame):
    self.count = 0
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
        self.RefByNum.insert(0, "Reference by number")
        self.RefByNum.place(x=170, y=90)

        ConfSelection = Button(self.master, text="Press to confirm", command=self.GetText)
        ConfSelection.place(x=300, y=90)

    def GetText(self):
        self.text = self.RefByNum.get()
        self.WriteToCommand()
        self.CreateControlButton()

    def WriteToCommand(self, argument = None):
        if argument is None:
            with open('commands.txt', 'w') as f:
                f.write(self.text)

            self.P.ReadFromCommand()
        elif argument is not None:
            if argument == "p" and self.count == 0:
                self.PlayBtn.config(text="Pause")
                self.count += 1

            elif arguemtn == "p" and self.count == 1:
                self.PlayBtn.config(text="Play")
                self.count -= 1

            else:
                with open('commands.txt', 'w') as f:
                    f.write(argument + "\n")
                    f.write("PlayerControl")

            self.P.ReadFromCommand()

    def CreateControlButton(self):
        self.PlayBtn = Button(self.master, text="Play", fg="green", command=lambda: self.WriteToCommand("p"))
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
    sys.exit()
