import subprocess
import time
from dataclasses import dataclass, field


@dataclass(slots=True)
class Benchmark:
    values: list[str]
    algorithms: list[str]
    progress: bool = False
    valuesizes: list[int] = field(init=False)
    results: dict[str, list[float]] = field(init=False)

    def __post_init__(self):
        self.valuesizes = [len(value.split(",")) for value in self.values]
        self.results = {}

    def run_with(self, binpath: str):
        for algorithm in self.algorithms:
            if algorithm not in self.results:
                self.results[algorithm] = []

            for value_idx in range(len(self.values)):
                if self.progress:
                    print(
                        f"Running {algorithm} with size {self.valuesizes[value_idx]}..."
                    )
                start = time.monotonic_ns()
                subprocess.run(
                    [binpath, algorithm, self.values[value_idx]]
                ).check_returncode()
                end = time.monotonic_ns()

                self.results[algorithm].append((end - start) / 1_000_000)


def get_binpath(lang: str) -> str:
    LANG_TO_BIN = {
        "c": "c/",
        "c++": "cpp/",
        "rust-debug": "rust/target/debug/rustalgo",
        "rust-release": "rust/target/release/rustalgo",
    }

    return LANG_TO_BIN[lang.lower()]


def get_algorithms(complexity: str) -> list[str]:
    COMPLEXITY_TO_ALGO = {"quadratic": ["bubble-sort"], "linear-log": ["heap-sort"]}

    return COMPLEXITY_TO_ALGO[complexity]
