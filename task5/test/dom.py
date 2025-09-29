import json
import subprocess
import sys

import networkx as nx
import pydot

src = sys.argv[1]
cfg_json = subprocess.check_output(
    f"bril2json < {src} | ../target/release/cfg", shell=True, text=True
)
dom_json = subprocess.check_output(
    f"bril2json < {src} | ../target/release/dom", shell=True, text=True
)

cfg_json = json.loads(cfg_json)
dom_json = json.loads(dom_json)

for f in cfg_json.keys():
    cfg_v = cfg_json[f]
    dom_v = dom_json[f]

    (cfg_dot,) = pydot.graph_from_dot_data(cfg_v)
    cfg = nx.DiGraph(nx.nx_pydot.from_pydot(cfg_dot))

    entry = [b for b in cfg.nodes() if b.startswith("0: ")]
    if not entry:
        continue

    entry = entry[0]

    for b, as_ in dom_v.items():
        for a in as_:
            assert all([a in p for p in nx.all_simple_paths(cfg, entry, b)])

print(f"{src}: OK")
