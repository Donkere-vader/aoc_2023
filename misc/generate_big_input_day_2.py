import random

with open("input_big.txt", "w") as f:
    for i in range(1_000_000):
        cube_sets = [[
            f"{random.randint(1, 100)} green, {random.randint(1, 100)} blue, {random.randint(1, 100)} red"
        ] for _ in range(random.randint(1, 5))]
        f.write(f"Game {i + 1}: {'; '.join([', '.join(s) for s in cube_sets])}\n")
