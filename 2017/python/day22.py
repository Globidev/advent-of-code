from lib import readinput, apply_n
my_input = readinput(__file__)

from collections import namedtuple, defaultdict
from enum import Enum

class Direction(Enum):
    Up = 0
    Right = 1
    Down = 2
    Left = 3

Virus = namedtuple('Virus', ['position', 'direction', 'infected_count'])

INFECTED = '#'
CLEAN = '.'
WEAKENED = 'W'
FLAGGED = 'F'

TURN = lambda dd: lambda dir: Direction((dir.value + dd) % len(Direction))
LEFT = TURN(-1)
RIGHT = TURN(+1)
DONT_TURN = TURN(0)
REVERSE = TURN(2)

TURN_LOGIC = {
    INFECTED: RIGHT,
    CLEAN: LEFT,
    WEAKENED: DONT_TURN,
    FLAGGED: REVERSE,
}

MOVE = lambda dx, dy: lambda x, y: (x + dx, y + dy)

MOVE_LOGIC = {
    Direction.Up:    MOVE(+0, -1),
    Direction.Right: MOVE(+1, +0),
    Direction.Down:  MOVE(+0, +1),
    Direction.Left:  MOVE(-1, +0),
}

NODE_TRANSITIONS_P1 = {
    CLEAN: INFECTED,
    INFECTED: CLEAN,
}

NODE_TRANSITIONS_P2 = {
    CLEAN: WEAKENED,
    WEAKENED: INFECTED,
    INFECTED: FLAGGED,
    FLAGGED: CLEAN,
}

def cluster_state(cluster_center):
    height = len(cluster_center)
    width = len(cluster_center[0])
    cluster = defaultdict(lambda: CLEAN)

    for y, row in enumerate(cluster_center):
        for x, node in enumerate(row):
            cluster[(x, y)] = node

    initial_position = (width // 2, height // 2)
    initial_virus = Virus(initial_position, Direction.Up, 0)

    return (initial_virus, cluster)

def burst(state, node_transitions):
    virus, cluster = state
    current_node = cluster[virus.position]

    new_direction = TURN_LOGIC[current_node](virus.direction)
    new_position = MOVE_LOGIC[new_direction](*virus.position)
    new_node_state = node_transitions[current_node]
    new_infected_count = virus.infected_count + (new_node_state == INFECTED)

    cluster[virus.position] = new_node_state

    return (
        Virus(new_position, new_direction, new_infected_count),
        cluster
    )

def day_22_1(cluster_center):
    initial_state = cluster_state(cluster_center)
    burst_p1 = lambda state: burst(state, NODE_TRANSITIONS_P1)
    final_virus, _ = apply_n(burst_p1, 10000, initial_state)

    return final_virus.infected_count

def day_22_2(cluster_center):
    initial_state = cluster_state(cluster_center)
    burst_p2 = lambda state: burst(state, NODE_TRANSITIONS_P2)
    final_virus, _ = apply_n(burst_p2, 10000000, initial_state)

    return final_virus.infected_count

cluster_center = my_input.split('\n')

print(f'part 1: {day_22_1(cluster_center)}')
print(f'part 2: {day_22_2(cluster_center)}')
