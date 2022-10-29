import itertools
import copy

with open('day21/input.txt', 'r') as fp:
# with open('day21/testinput.txt', 'r') as fp:
    lines = [x.strip().split() for x in fp.readlines()]

class Player:
    def __init__(self, start_pos) -> None:
        self.start_pos = start_pos
        self.total_pos = start_pos - 1
        self.current_pos = start_pos
        self.pos_list = []
        self.score = 0

    def move(self, dists):
        self.total_pos = self.total_pos + sum(dists)
        self.current_pos = self.total_pos % 10 + 1
        self.pos_list.append(self.current_pos)
        self.score += self.current_pos
        return self.score

players = [Player(int(x[-1])) for x in lines]

winner = -1
rolls = 0
while winner == -1:
    for i, p in enumerate(players):
        dists = [(rolls+x)%100 +1 for x in range(3)]
        score = p.move(dists)
        rolls += 3
        if score >= 1000:
            winner = i
            break

print(f"Player {winner + 1} won with {players[winner].score} points")
print(f"Result Part 1: {players[1-winner].score * rolls}")

