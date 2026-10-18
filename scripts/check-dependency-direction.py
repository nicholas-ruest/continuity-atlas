#!/usr/bin/env python3
import json,subprocess,sys
m=json.loads(subprocess.check_output(["cargo","metadata","--locked","--no-deps","--format-version","1"]))
workspace={p["name"] for p in m["packages"]}
reverse={n:0 for n in workspace}
edges=0
for p in m["packages"]:
 for d in p["dependencies"]:
  if d["name"] in workspace:
   reverse[d["name"]]+=1;edges+=1
roots={"ca-cli","ca-evaluation"}
bad=sorted(n for n,c in reverse.items() if c==0 and n not in roots)
print(json.dumps({"packages":len(workspace),"edges":edges,"zero_reverse_callers":bad},sort_keys=True))
sys.exit(bool(bad or len(workspace)<20))
