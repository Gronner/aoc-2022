import re
import sys
import functools

translation_to_hashable = {}
flow_rates = {}
successors = {}
cache = {}

def compute_max_flow(position, visited, time, part2):
    if time == 0:
        if part2:
            return compute_max_flow(translation_to_hashable["AA"], visited, 26, False)
        else:
            return 0

    cached = (position, visited, time, part2)
    if cached in cache:
        return cache[cached]

    result = 0

    cachable_visited = (1 << position)

    cur_flow_rate = flow_rates[position]

    is_not_visited = 0 == (visited & cachable_visited)
    if is_not_visited and cur_flow_rate > 0:
        new_visited = visited | cachable_visited
        result = max(result, (time - 1) * cur_flow_rate + compute_max_flow(position, new_visited, time - 1, part2))

    for successor in successors[position]:
        result = max(result, compute_max_flow(successor, visited, time - 1, part2))

    cache[cached] = result

    return result

def main(): 
    with open("input/input16.txt", 'r') as input:
        i = 0
        for line in input.readlines():
            resolved = re.match("Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", line)
            translation_to_hashable[resolved.group(1)] = i
            i += 1

    with open("input/input16.txt", 'r') as input:
        for line in input.readlines():
            resolved = re.match("Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)", line)
            flow_rates[translation_to_hashable[resolved.group(1)]] = int(resolved.group(2))
            successors[translation_to_hashable[resolved.group(1)]] = []
            for successor in resolved.group(3).split(', '):
                successors[translation_to_hashable[resolved.group(1)]].append(translation_to_hashable[successor])

    print("Part 1:", compute_max_flow(translation_to_hashable["AA"], 0, 30, False))
    print("Part 2:", compute_max_flow(translation_to_hashable["AA"], 0, 26, True))


if __name__ == "__main__":
    sys.setrecursionlimit(sys.getrecursionlimit() * 1000)
    main()



