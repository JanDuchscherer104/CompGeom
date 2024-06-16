import subprocess
import time
from dataclasses import dataclass, field
from typing import Any, Dict, List, Tuple

import numpy as np
import pandas as pd
from pandarallel import pandarallel

pandarallel.initialize(progress_bar=True)


@dataclass
class QhullAnalyzerConfig:
    """
    Configuration for the experiment, including all sweeps.
    """

    dimensions: List[int] = field(default_factory=lambda: list(range(2, 9)))
    num_points: List[int] = field(default_factory=lambda: [100, 500, 1000, 5000, 10000])
    qhull_options: List[str] = field(
        default_factory=lambda: ["-V Qv Qt", "-V Qv QJ", "-V Qv Tv"],
    )
    rbox_option: List[str] = field(
        default_factory=lambda: [
            "c",
        ],
    )
    is_parallel: bool = True

    def generate_params(self) -> pd.DataFrame:
        params = [
            (dim, points, qhull_opt, f"{rbox_opt} D{dim}")
            for dim in self.dimensions
            for points in self.num_points
            for rbox_opt in self.rbox_option
            for qhull_opt in self.qhull_options
        ]
        return pd.DataFrame(
            params,
            columns=["dimensions", "num_points", "qhull_options", "rbox_options"],
        )

    def setup(self) -> "Experiment":
        return Experiment(self)


class RuntimeTracker:
    def __init__(self):
        self.runtimes: List[Dict[str, Any]] = []  # type: ignore

    def add_runtime(
        self,
        dimensions: int,
        num_points: int,
        runtime: float,
        rbox_options: str,
        qhull_options: str,
    ) -> None:
        self.runtimes.append(
            {
                "dimensions": dimensions,
                "num_points": num_points,
                "runtime": runtime,
                "rbox_options": rbox_options,
                "qhull_options": qhull_options,
            }
        )

    def to_dataframe(self) -> pd.DataFrame:
        return pd.DataFrame(self.runtimes)


class QhullHandler:
    def run_qhull(self, points: np.ndarray, qhull_options: str) -> Tuple[float, str]:
        try:
            start_time = time.time()
            input_data = f"{points.shape[1]} {points.shape[0]}\n" + "\n".join(
                " ".join(map(str, point)) for point in points
            )
            process = subprocess.run(
                ["qconvex"] + qhull_options.split(),
                input=input_data,
                capture_output=True,
                text=True,
                check=True,
            )
            return time.time() - start_time, process.stdout
        except subprocess.CalledProcessError as e:
            return -1, f"Error: {e}"


class Experiment:
    def __init__(self, config: QhullAnalyzerConfig):
        self.config = config
        self.qhull_handler = QhullHandler()
        self.runtime_tracker = RuntimeTracker()

    def generate_points(self, num_points: int, rbox_options: str) -> np.ndarray:
        result = subprocess.run(
            ["rbox", f"{num_points}"] + rbox_options.split(),
            capture_output=True,
            text=True,
            check=True,
        )

        lines = [
            line
            for line in result.stdout.splitlines()
            if line.strip() and not line.startswith("#")
        ]
        assert (
            len(lines) == int(lines[1]) + 2
        ), f"Unexpected rbox output for {lines[0]}, len(lines)={len(lines)} != {lines[1]} + 2"
        points = np.loadtxt(lines[2:])
        assert isinstance(points, np.ndarray)

        return points

    def run(self) -> pd.DataFrame:
        """
        Runs the experiment, generating points, running Qhull, and tracking runtimes.
        """
        params_df = self.config.generate_params().sample(frac=1).reset_index(drop=True)

        if self.config.is_parallel:
            results = params_df.parallel_apply(self._worker, axis=1)
            for result in results:
                self.runtime_tracker.add_runtime(*result)
        else:
            params_df.apply(self._worker, axis=1)

        return self.runtime_tracker.to_dataframe()

    def _worker(self, row: pd.Series) -> Tuple[int, int, float, str, str]:
        points = self.generate_points(row["num_points"], row["rbox_options"])
        runtime, _ = self.qhull_handler.run_qhull(points, row["qhull_options"])
        return (
            row["dimensions"],
            row["num_points"],
            runtime,
            row["rbox_options"],
            row["qhull_options"],
        )
