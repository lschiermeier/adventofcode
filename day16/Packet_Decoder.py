with open('day16/input.txt', 'r') as fp:
# with open('day16/testinput.txt', 'r') as fp:
    intList = [int(x,16) for x in fp.readline().strip()]

binString = "".join(["{:04b}".format(x) for x in intList])

def str2num(idx, length, string):
    return int(string[idx:idx+length],2), idx + length

def parseLiteral(idx, string) -> int:
    cont = True
    num = 0
    while cont:
        cont = string[idx] == '1'
        tempNum, idx = str2num(idx+1,4,string)
        num += tempNum
        num *= 16 if cont else 1
    return num, idx

class Packet:
    def __init__(self, idx, string) -> None:
        MinPacketSize = 11
        self.version, idx = str2num(idx,3,string)
        self.typeID,  idx = str2num(idx,3,string)
        self.subPackets = []
        if self.typeID == 4:
            self.literal, idx = parseLiteral(idx,string)
        else:
            totalLength = string[idx] == '0'
            nextPackets, idx = str2num(idx + 1, 15 if totalLength else 11, string)
            if totalLength:
                nextPackets += idx
                while nextPackets > idx:
                    newPacket = Packet(idx,string)
                    self.subPackets.append(newPacket)
                    idx = newPacket.end_idx
                    if nextPackets - idx < MinPacketSize:
                        idx = nextPackets
            else:
                for i in range(nextPackets):
                    newPacket = Packet(idx,string)
                    self.subPackets.append(newPacket)
                    idx = newPacket.end_idx
        self.end_idx = idx

    def sumAllVers(self):
        return self.version + sum(map(Packet.sumAllVers,self.subPackets))

binLen = len(binString)
godPacket = Packet(0,binString)

print(f"Result Part 1: {godPacket.sumAllVers()}")

pass