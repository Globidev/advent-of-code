from lib import readinput
my_input = readinput(__file__, strip=False)

VERTICAL_PATH = '|'
HORIZONTAL_PATH = '-'
INTERSECTION = '+'
VOID = ' '

def packet_steps(diagram):
    y = 0 # We start at the top of the diagram
    x = diagram[y].index(VERTICAL_PATH)
    dx, dy = 0, 1
    width, height = len(diagram[0]), len(diagram)

    def at(x, y):
        in_bound = 0 <= x < width and 0 <= y < height
        return diagram[y][x] if in_bound else VOID

    def turn(x, y, dx, dy):
        if dx:
            new_dy = 1 if at(x, y + 1) != VOID else -1
            return (0, new_dy)
        else:
            new_dx = 1 if at(x + 1, y) != VOID else -1
            return (new_dx, 0)

    while at(x, y) != VOID:
        yield (x, y)

        if at(x, y) == INTERSECTION:
            dx, dy = turn(x, y, dx, dy)

        x += dx
        y += dy

def day_19_1(diagram):
    return ''.join(
        diagram[y][x]
        for x, y in packet_steps(diagram)
        if diagram[y][x] not in [VERTICAL_PATH, HORIZONTAL_PATH, INTERSECTION]
    )

def day_19_2(inp):
   return sum(1 for _ in packet_steps(diagram))

diagram = my_input.split('\n')

print(f'part 1: {day_19_1(diagram)}')
print(f'part 2: {day_19_2(diagram)}')
