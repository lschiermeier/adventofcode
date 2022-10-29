import numpy as np
import re

with open('day17/input.txt', 'r') as fp:
# with open('day17/testinput.txt', 'r') as fp:
    input = re.split(": |, |\.\.|=",fp.readline().strip())

def line2dict(line):
    dict = {}
    dict["x_min"] = int(line[2])
    dict["x_max"] = int(line[3])
    dict["y_min"] = int(line[5])
    dict["y_max"] = int(line[6])
    return dict

class Probe:
    def __init__(self, x_vel, y_vel) -> None:
        self.start_vel = (x_vel, y_vel)
        self.x_vel = x_vel
        self.y_vel = y_vel
        self.x_pos = 0
        self.y_pos = 0
        self.max_y_pos = 0

    def doStep(self):
        self.x_pos += self.x_vel
        self.y_pos += self.y_vel
        self.max_y_pos = max(self.max_y_pos, self.y_pos)
        if self.x_vel > 0:
            self.x_vel -= 1
        elif self.x_vel < 0:
            self.x_vel += 1
        self.y_vel -= 1
        return self.x_pos, self.y_pos

target = line2dict(input)
valid_trajectories = []

for x_vel in range(1,320):
    for y_vel in range(-80,150):
        new_trajectory = Probe(x_vel,y_vel)
        x_pos, y_pos = (0,0)
        while x_pos < target["x_max"] and y_pos > target["y_min"]:
            x_pos, y_pos = new_trajectory.doStep()
            if x_pos >= target["x_min"] and x_pos <= target["x_max"]:
                if y_pos >= target["y_min"] and y_pos <= target["y_max"]:
                    valid_trajectories.append(new_trajectory)
                    break
        else:
            # invalid Trajectory
            continue

valid_trajectories.sort(key=(lambda x: x.max_y_pos), reverse=True)
mostStylish = valid_trajectories[0]

print(f"Result Part 1: {mostStylish.max_y_pos}")
print(f"Start Vel = {mostStylish.start_vel}")

print(f"Result Part 2: {len(valid_trajectories)}")
