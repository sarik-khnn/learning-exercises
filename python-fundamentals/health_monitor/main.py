from collections import defaultdict
from collections import deque


def system_health(logs: list) -> dict:
    queue = deque()
    data = defaultdict(set)
    total = 0
    for log in logs:
        total = total + 1
        queue.append(log)
        log = queue.popleft()
        if log[2] == "ERROR":
            data[log[2]].add(log[1])

    report = {"total": total, "data": data}
    return report


logs = [
    ("10:00", "srv-1", "ERROR", "Disk Full"),
    ("10:01", "srv-2", "INFO", "User Login"),
    ("10:02", "srv-1", "ERROR", "CPU Spike"),
    ("10:03", "srv-3", "ERROR", "DB Connection Fail"),
    ("10:04", "srv-2", "ERROR", "Disk Full"),
]
result = system_health(logs)
print("[SYSTEM HEALTH REPORT]")
for level, server in result["data"].items():
    print(f"{level} Summary:")
    print(f"{level} {len(server)} logs total. Affected servers: {server}")
    print()
print("Log Queue Status:")
print(f"Processed {result['total']} logs successfully.")
 