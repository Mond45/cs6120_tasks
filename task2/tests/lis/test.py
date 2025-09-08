import random
import subprocess

from tqdm.contrib.concurrent import process_map

N = 100000


def test(vals: list[str]):
    output1 = subprocess.run(
        ["brili"] + vals,
        input=open("lis.json").read(),
        capture_output=True,
        text=True,
    ).stdout.strip()

    output2 = subprocess.run(
        "./lis", input=" ".join(vals), capture_output=True, text=True
    ).stdout.strip()

    assert output1 == output2


process_map(
    test,
    [[str(random.randint(0, 10)) for _ in range(8)] for _ in range(N)],
    max_workers=16,
)
