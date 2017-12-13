from lib import readinput, scan
my_input = readinput(__file__)

from collections import namedtuple
from itertools import count

Layer = namedtuple('Layer', ['depth', 'range'])

def period(layer):
    return 2 * (layer.range - 1)

def parse_layer(layer_str):
    depth_str, range_str = layer_str.split(': ')
    return Layer(int(depth_str), int(range_str))

def day_13_1(firewall):
    severity = sum(
        layer.depth * layer.range
        for layer in firewall
        if layer.depth % period(layer) == 0
    )

    return severity

def day_13_2(firewall):
    return next(
        delay for delay in count()
        if all(
            (layer.depth + delay) % period(layer) != 0
            for layer in firewall
        )
    )

firewall = [
    parse_layer(layer_str)
    for layer_str in my_input.split('\n')
]

print(f'part 1: {day_13_1(firewall)}')
print(f'part 2: {day_13_2(firewall)}')
