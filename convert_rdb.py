import csv

import redis

# user_1
#    ts1 -> "; Ukrainian"
#    ts2 -> "injury; Ukrainian"
#    ts3 -> "injury->head; Ukrainian"
#    ts4 -> "; Ukrainian"
# user_2
#    ts1 -> "; Ukrainian"
#    ts2 -> "headache; Ukrainian"


def main():
    r = redis.Redis(host="localhost", port=6379, db=0)
    keys = [
        k
        for k in map(lambda s: s.decode("utf-8"), r.keys())
        if k.startswith("user_") and k != "user_ids"
    ]

    data = []
    for k in keys:
        hkeys = r.hkeys(k)
        for hk in map(lambda s: s.decode("utf-8"), hkeys):
            v = r.hget(k, hk).decode("utf-8")
            print(f"{k.removeprefix('user_')=}, {hk=}, {v=}")
            data.append([k.removeprefix("user_"), hk, v])

    with open("state.csv", "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["user_id", "timestamp", "state"])

        writer.writerows(data)


if __name__ == "__main__":
    main()
