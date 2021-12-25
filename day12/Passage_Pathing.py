with open('day12/input.txt', 'r') as fp:
# with open('day12/testinput.txt', 'r') as fp:
    lines = [x.strip().split("-") for x in fp.readlines()]

class Node:
    def __init__(self,name:str) -> None:
        self.name = name
        self.isBig = name.isupper()
        self.edges = []

    def add_edge(self, other:'Node'):
        self.edges.append(other)
        other.edges.append(self)

class Path:
    def __init__(self,start:Node):
        self.start = start
        self.nodes = [start]
    def add_node(self, next:Node) -> int:
        if next.isBig:
            self.nodes.append(next)
            return 1
        elif next in self.nodes:
            return 0
        else:
            self.nodes.append(next)
            return True
    def copy(self):
        new = Path(self.start)
        new.nodes = self.nodes.copy()
        return new

web = {}
for a, b in lines:
    if not a in web.keys():
        web[a] = Node(a)
    if not b in web.keys():
        web[b] = Node(b)
    web[a].add_edge(web[b])

def expand(path:Path):
    next_Nodes = path.nodes[-1].edges
    paths = [path.copy() for e in next_Nodes]
    extended_ps = list(map(Path.add_node, paths, next_Nodes))
    return paths[extended_ps]

all_paths = []
done = False
while not done:
    
    pass