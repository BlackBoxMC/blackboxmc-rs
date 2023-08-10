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
    if arg in cur:
        cur = cur[arg]
        should_print = True
        past.append(arg)
    else:
        should_print = False
        print(arg+" not in "+":".join(past))

if should_print:
    print(cur)