from lib import readinput
my_input = readinput(__file__)

from collections import namedtuple
from functools import reduce

class StreamState:

    def __init__(self):
        self.in_garbage = False
        self.skip_next = False
        self.garbage_count = 0
        self.group_nesting = 0
        self.group_score = 0

def process_char(state, char):
    if state.skip_next:
        state.skip_next = False
    elif char == '!':
        state.skip_next = True
    elif state.in_garbage:
        if char == '>':
            state.in_garbage = False
        else:
            state.garbage_count += 1
    else:
        if char == '{':
            state.group_nesting += 1
        elif char == '}':
            state.group_score += state.group_nesting
            state.group_nesting -= 1
        elif char == '<':
            state.in_garbage = True

    return state

def day_9_1(stream):
    final_state = reduce(process_char, stream, StreamState())
    return final_state.group_score

def day_9_2(stream):
    final_state = reduce(process_char, stream, StreamState())
    return final_state.garbage_count

print(f'part 1: {day_9_1(my_input)}')
print(f'part 2: {day_9_2(my_input)}')
