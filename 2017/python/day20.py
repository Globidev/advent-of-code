from lib import readinput
my_input = readinput(__file__)

from collections import namedtuple, Counter
from itertools import combinations

Position = namedtuple('Position', ['x', 'y', 'z'])
Velocity = namedtuple('Velocity', ['dx', 'dy', 'dz'])
Acceleration = namedtuple('Acceleration', ['ax', 'ay', 'az'])

Particle = namedtuple('Particle', ['position', 'velocity', 'acceleration'])

ORIGIN = (0, 0, 0)

def manhattan(vector1, vector2):
    return sum(abs(c1 - c2) for c1, c2 in zip(vector1, vector2))

def parse_particle(particle_str):
    position_str, velocity_str, acceleration_str = particle_str.split(', ')
    position = Position(*map(int, position_str[3:-1].split(',')))
    velocity = Velocity(*map(int, velocity_str[3:-1].split(',')))
    acceleration = Acceleration(*map(int, acceleration_str[3:-1].split(',')))

    return Particle(position, velocity, acceleration)

def day_20_1(particles):
    i, _ = min(
        enumerate(particles),
        key=lambda t: manhattan(t[1].acceleration, ORIGIN)
    )

    return i

def will_collide(p1, p2):
    distance = manhattan(p1.position, p2.position)
    while True:
        if distance == 0:
            return True
        p1 = step(p1)
        p2 = step(p2)
        new_distance = manhattan(p1.position, p2.position)
        if new_distance >= distance:
            return False
        distance = new_distance

def step(particle):
    new_velocity = Velocity(*(dc + ac
        for dc, ac in zip(particle.velocity, particle.acceleration)))
    new_position = Position(*(pc + dc
        for pc, dc in zip(particle.position, new_velocity)))

    return Particle(new_position, new_velocity, particle.acceleration)

def step_all(particles):
    new_particles = [step(particle) for particle in particles]
    count_by_position = Counter(particle.position for particle in new_particles)

    return [
        particle for particle in new_particles
        if count_by_position[particle.position] == 1
    ]

def day_20_2(particles):
    while any(will_collide(p1, p2) for p1, p2 in combinations(particles, 2)):
        particles = step_all(particles)

    return len(particles)

particles = [
    parse_particle(particle_str)
    for particle_str in my_input.split('\n')
]

print(f'part 1: {day_20_1(particles)}')
print(f'part 2: {day_20_2(particles)}')
