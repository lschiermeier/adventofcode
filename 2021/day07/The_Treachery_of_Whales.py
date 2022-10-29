from itertools import repeat

with open('day07/input.txt', 'r') as fp:
    crab_positions = list(map(int,fp.readline().strip().split(",")))

def calcLinearCost(starts, target):
    cost = 0
    for s in starts:
        cost += abs(s - target)
    return cost

def calcQuadraticCost(starts, target):
    cost = 0
    for s in starts:
        dist = abs(s - target)
        cost += int((dist * (dist+1))/2)
    return cost

linear_costs = list(map(calcLinearCost,repeat(crab_positions),range(max(crab_positions))))
print(f"Result part 1: {min(linear_costs)}")

quadratic_costs = list(map(calcQuadraticCost,repeat(crab_positions),range(max(crab_positions))))
print(f"Result part 2: {min(quadratic_costs)}")

