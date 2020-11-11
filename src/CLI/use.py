from script import Player
import sys

def main():
    P1 = Player()
    guard = True
    while guard:
        P1.Play()
        playing = True

        while playing:
            P1.PlayerControl()

while True:
    main()

sys.exit()
