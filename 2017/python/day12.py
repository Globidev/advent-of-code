from lib import readinput, scan
my_input = readinput(__file__)

from collections import defaultdict, namedtuple

Relation = namedtuple('Relation', ['node_from', 'node_tos'])

def parse_relation(relation_str):
    from_str, tos_list = relation_str.split(' <-> ')
    tos_str = tos_list.split(', ')

    return Relation(int(from_str), [int(n_str) for n_str in tos_str])

def make_graph(relations):
    graph = defaultdict(set)

    for relation in relations:
        graph[relation.node_from].union(relation.node_tos)
        for node_to in relation.node_tos:
            graph[node_to].add(relation.node_from)

    return graph

def dfs(graph, node, visited=set()):
    if node in visited:
        return

    yield node
    for child in graph[node]:
        yield from dfs(graph, child, visited | {node})

def day_12_1(graph):
    return sum(1 for _ in dfs(graph, 0))

def day_12_2(graph):
    unmarked = set(node for node in graph)
    groups = 0
    while unmarked:
        groups += 1
        some_node = next(iter(unmarked))
        connected = set(node for node in dfs(graph, some_node))
        unmarked -= connected

    return groups

relations = (
    parse_relation(relation_str)
    for relation_str in my_input.split('\n')
)

graph = make_graph(relations)

print(f'part 1: {day_12_1(graph)}')
print(f'part 2: {day_12_2(graph)}')
