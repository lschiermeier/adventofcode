from numpy.core.fromnumeric import nonzero
from numpy.core.numeric import count_nonzero
from rich.traceback import install
import numpy as np
install(show_locals=True)

def hasWon(mask: np.array):
    s = mask.shape
    for x in range(s[0]):
        hor = np.count_nonzero(mask[x,:])
        if hor == s[0]:
            return True
    for y in range(s[1]):
        vert = np.count_nonzero(mask[:,y])
        if vert == s[1]:
            return True
    return False


with open('day4/input.txt', 'r') as fp:
    lines = [x.strip() for x in fp.readlines()]

draws = list(map(int,lines[0].split(",")))

all_boards = [list(map(int,x.split())) for x in lines[2:] if len(x) > 0 ]
board_num = int(len(all_boards)/5)
boards = [np.asarray(all_boards[x*5:x*5+5]) for x in range(board_num)]
drawn_boards = [np.zeros_like(x) for x in boards]

winning_rounds = [0] * board_num
winning_numbers = [0] * board_num
for round,d in enumerate(draws):
    for idx, board in enumerate(boards):
        if winning_rounds[idx] > 0:
            continue
        drawn_boards[idx] += np.where(board == d,board,np.zeros_like(board))
        if hasWon(drawn_boards[idx]):
            winning_rounds[idx] = round
            winning_numbers[idx] = d

first_winning_round = min(winning_rounds)
first_winner = winning_rounds.index(first_winning_round)

print(f"Board {first_winner} won first in Round {first_winning_round}")
print(drawn_boards[first_winner])
unmarked_sum = np.sum(
                    np.where(boards[first_winner] != drawn_boards[first_winner],
                        boards[first_winner],
                        np.zeros_like(boards[first_winner]))
                    )
print(f"With a Score of {unmarked_sum} * {winning_numbers[first_winner]} = {unmarked_sum * winning_numbers[first_winner]} ")

## part 2

last_winning_round = max(winning_rounds)
last_winner = winning_rounds.index(last_winning_round)

print(f"Board {last_winner} won last in Round {last_winning_round}")
print(drawn_boards[last_winner])
unmarked_sum = np.sum(
                    np.where(boards[last_winner] != drawn_boards[last_winner],
                        boards[last_winner],
                        np.zeros_like(boards[last_winner]))
                    )
print(f"With a Score of {unmarked_sum} * {winning_numbers[last_winner]} = {unmarked_sum * winning_numbers[last_winner]} ")


print("end")
