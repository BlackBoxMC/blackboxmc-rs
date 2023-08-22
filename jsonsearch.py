#!/usr/bin/python
import json 
import sys

args = sys.argv[1].split(":")
print(args)

f = open("./spigot.json")
past = ["spigot.json"]
should_print = False
cur = json.loads(f.read())
for arg in args:
    if arg == "_keys_":
        cur = cur.keys()
        should_print = True
        past.append(arg)
    elif arg.startswith("("):
        parts = arg.replace("(","").replace(")","").split("=")
        curr = list(filter(lambda f: f[parts[0]] == parts[1], cur))
        if len(curr) >= 1:
            cur = curr
            should_print = True
            past.append(arg)
        else:
            should_print = False
            print(arg+" not in "+":".join(past))
    else:
        if arg in cur:
            cur = cur[arg]
            should_print = True
            past.append(arg)
        else:
            should_print = False
            print(arg+" not in "+":".join(past))

if should_print:
    print(json.dumps(cur,indent=2))