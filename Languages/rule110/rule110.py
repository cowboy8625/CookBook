import time
def pair_up(l):
    return [(l[(i-1)], a, l[(i+1)%len(l)]) for i, a in enumerate(l)]

def check(items):
    match items:
        case (" ", " ", " ") |\
             ("#", " ", " ") |\
             ("#", "#", "#"):
            return " "
        case ("#", "#", " ") |\
             ("#", " ", "#") |\
             (" ", "#", "#") |\
             (" ", "#", " ") |\
             (" ", " ", "#"):
            return "#"
        case _:
            assert False, "Unreachable Pattern"

width = 100
world = [" " if i < width-1 else "#" for i in range(width)]
while True:
    print("".join(world))
    world = list(map(check, pair_up(world)))
    time.sleep(0.09)
