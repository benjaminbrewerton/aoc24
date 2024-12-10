import sys
from pathlib import Path

def sum_distance(filepath: Path):
    left = []
    right = []
    for line in filepath.read_text().splitlines():
        if not line:
            continue
        nums = line.strip().split()
        left.append(int(nums[0]))
        right.append(int(nums[-1]))
    return sum(abs(x - y) for x, y in zip(sorted(left), sorted(right)))

if __name__ == "__main__":
    print(f'Got distance {sum_distance(Path(sys.argv[1]).resolve())}')