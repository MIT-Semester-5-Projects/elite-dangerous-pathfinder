import json

path = [10477373803, 8605134754514, 13866167772569, 6681123623626]
map_data = json.load(open("./systems_1week.json"))
for i in path:
    for j in map_data:
        if j["id64"] == i:
            print(f"{j['name']} -> ")
