from lib import readinput
my_input = readinput(__file__)

def day_1_x(steps_forward, seq):
    return sum(
        digit if seq[(i + steps_forward) % len(seq)] == digit else 0
        for i, digit in enumerate(seq)
    )

day_1_1 = lambda seq: day_1_x(1, seq)
day_1_2 = lambda seq: day_1_x(len(seq) // 2, seq)

captcha = [int(c) for c in my_input]

print(f'part 1: {day_1_1(captcha)}')
print(f'part 2: {day_1_2(captcha)}')
