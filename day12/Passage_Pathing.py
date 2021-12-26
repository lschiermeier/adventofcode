
with open('day12/input.txt', 'r') as fp:
# with open('day12/testinput.txt', 'r') as fp:
# with open('day12/smallinput.txt', 'r') as fp:
    lines = [x.strip().split("-") for x in fp.readlines()]

class Node:
    def __init__(self,name:str) -> None:
        self.name = name
        self.isBig = name.isupper()
        self.isTerminal = name in ["end", "start"]
        self.edges = []

    def add_edge(self, other:'Node'):
        self.edges.append(other)
        other.edges.append(self)

class Path:
    def __init__(self,start:Node,usedJoker):
        self.start = start
        self.nodes = [start]
        self.usedJoker = usedJoker
    def add_node(self, next:Node) -> int:
        """ returns true on success """
        if next.isBig:
            self.nodes.append(next)
            return True
        elif next in self.nodes:
            if not self.usedJoker and not next.isTerminal:
                self.nodes.append(next)
                self.usedJoker = True
                return True
            else:
                return False
        else:
            self.nodes.append(next)
            return True
    def copy(self):
        new = Path(self.start, self.usedJoker)
        new.nodes = self.nodes.copy()
        return new
    def is_ended(self):
        return self.nodes[-1].name == "end"

def expand(path:Path):
    next_Nodes = path.nodes[-1].edges
    paths = [path.copy() for _ in next_Nodes]
    ext_path_sel = list(map(Path.add_node, paths, next_Nodes))
    return [p for p,sel in zip(paths,ext_path_sel) if sel]

web = {}
for a, b in lines:
    if not a in web.keys():
        web[a] = Node(a)
    if not b in web.keys():
        web[b] = Node(b)
    web[a].add_edge(web[b])

finished_paths = []
working_paths = [Path(web["start"], True)]
while working_paths:
    print(len(working_paths),len(finished_paths))
    new_paths = [new_p for old_p in working_paths for new_p in expand(old_p)]
    ended_sel = list(map(Path.is_ended, new_paths))
    finished_paths += [p for p,sel in zip(new_paths, ended_sel) if sel]
    working_paths = [p for p,sel in zip(new_paths, ended_sel) if not sel]

print(f"Result Part 1: {len(finished_paths)}")

finished_paths = []
working_paths = [Path(web["start"], False)]
while working_paths:
    print(len(working_paths),len(finished_paths))
    new_paths = [new_p for old_p in working_paths for new_p in expand(old_p)]
    ended_sel = list(map(Path.is_ended, new_paths))
    finished_paths += [p for p,sel in zip(new_paths, ended_sel) if sel]
    working_paths = [p for p,sel in zip(new_paths, ended_sel) if not sel]

print(f"Result Part 2: {len(finished_paths)}")