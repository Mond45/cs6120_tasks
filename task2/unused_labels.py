import json
from sys import stdin


def get_labels(instrs):
    for instr in instrs:
        if "label" in instr:
            yield instr["label"]


def get_used_labels(instrs):
    for instr in instrs:
        if "op" in instr and instr["op"] in ("br", "jmp"):
            yield from instr["labels"]


if __name__ == "__main__":
    program = json.load(stdin)
    functions = program["functions"]
    fn_instrs = [fn["instrs"] for fn in functions]

    for function, labels, used_labels in zip(
        functions,
        [set(get_labels(instrs)) for instrs in fn_instrs],
        [set(get_used_labels(instrs)) for instrs in fn_instrs],
    ):
        unused_labels = ", ".join(sorted(labels - used_labels))
        print(f'{function["name"]}: {unused_labels}')
