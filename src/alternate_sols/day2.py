import dataclasses
import sys
from pathlib import Path

MIN_DISTANCE = 1
MAX_DISTANCE = 3

def sign(a: int):
    return (a > 0) - (a < 0)

@dataclasses.dataclass
class Report:
    entries: list[int]
    dampened: bool = False
    dampened_idx: int = -1

    def entries_sign_cmp(self, a: int) -> tuple[int, int]:
        # Returns delta, sign
        fwd = a + 1
        if self.dampened:
            if self.dampened_idx == a:
                a += 1
                fwd += 1
            elif self.dampened_idx == fwd:
                fwd += 1
        cur_a = self.entries[a]
        fwd_a = self.entries[fwd]
        delta = fwd_a - cur_a
        return (delta, sign(delta))

    def validate_report(self, allow_dampening: bool = True) -> bool:
        idx = 0
        exp_sign = None
        while idx < len(self.entries) - 1:
            delta, new_sign = self.entries_sign_cmp(idx)
            delta_abs = abs(delta)
            if (delta_abs > MAX_DISTANCE or
                delta_abs < MIN_DISTANCE or
                (exp_sign is not None and new_sign != exp_sign)):
                if not allow_dampening:
                    return False
                try:
                    self.dampen(idx)
                except Exception:
                    return False
            idx += 1
            exp_sign = new_sign
        return True

    def dampen(self, idx: int):
        if not self.dampened:
            self.dampened = True
            self.dampened_idx = idx
            return
        raise Exception('Already Dampened')

def generate_reports(input: Path):
    for line in input.read_text('utf-8').splitlines():
        if not line:
            continue
        ints = [int(i.strip()) for i in line.split()]
        yield Report(ints)

if __name__ == "__main__":
    reports = list(generate_reports(Path(sys.argv[1]).resolve()))
    valid_reports = [report.validate_report() for report in reports]
    print('Number of safe reports is: '
          f'{len([valid_report for valid_report in valid_reports if valid_report])}')