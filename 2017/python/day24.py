from lib import readinput
my_input = readinput(__file__)

from collections import namedtuple

Component = namedtuple('Component', ['left', 'right'])
Bridge = namedtuple('Bridge', ['end', 'components'])

def parse_component(component_str):
    return Component(*map(int, component_str.split('/')))

def has_pin_port(component, port_no):
    return component.left == port_no or component.right == port_no

def strength(bridge):
    return sum(
        component.left + component.right
        for component in bridge.components
    )

def single_bridge(component):
    return Bridge(
        component.left if component.right == 0 else component.right,
        [component]
    )

def longest_bridges(bridge, components_left):
    connectable = [
        component for component in components_left
        if has_pin_port(component, bridge.end)
    ]

    if not connectable:
        yield bridge

    for component in connectable:
        new_end = (
            component.right
            if component.left == bridge.end
            else component.left
        )

        new_bridge = Bridge(new_end, bridge.components + [component])

        yield from longest_bridges(new_bridge, components_left - { component })

def day_24_1(bridges):
    return max(strength(bridge) for bridge in bridges)

def day_24_2(bridges):
    _, longest_bridge_strength = max(
        (len(bridge.components), strength(bridge))
        for bridge in bridges
    )

    return longest_bridge_strength

components = set(
    parse_component(component_str)
    for component_str in my_input.split('\n')
)

all_bridges = [bridges
    for component in components
    if has_pin_port(component, 0)
    for bridges in longest_bridges(
        single_bridge(component),
        components - { component }
    )
]

print(f'part 1: {day_24_1(all_bridges)}')
print(f'part 2: {day_24_2(all_bridges)}')
