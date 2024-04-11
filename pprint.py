import json

f = open("./spigot.json","r")
print(json.dumps(json.loads("".join(f.readlines())), sort_keys=True, indent=4))
