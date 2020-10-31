from script import Player

term = input("Video to search for: ")
P1 = Player(term)

while True:
    P1.main()
    P1.playerControl()
