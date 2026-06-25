from collections import defaultdict
from collections import deque


def system_health(logs: list) -> dict:
    queue = deque()
    grouping = defaultdict(set)
    t_p = 0
    t_e = 0
    for log in logs:
        t_p = t_p + 1
        queue.append(log)
        log = queue.popleft()
        if log[2] == "ERROR":
            t_e = t_e + 1
        grouping[log[2]].add(log[1])

    report = {"t_p": t_p, "t_e": t_e, "err": grouping["ERROR"]}
    return report


logs = [
    ("10:00", "srv-1", "ERROR", "Disk Full"),
    ("10:01", "srv-2", "INFO", "User Login"),
    ("10:02", "srv-1", "ERROR", "CPU Spike"),
    ("10:03", "srv-3", "ERROR", "DB Connection Fail"),
    ("10:04", "srv-2", "ERROR", "Disk Full"),
]
result = system_health(logs)
for levels in result.items():
    t_p = result.get("t_p")
    t_e = result.get("t_e")
    err = result.get("err")
print("[SYSTEM HEALTH REPORT]")
print("Error Summary:")
print(f"ERROR: {t_e} logs total. Affected servers: {err}")
print()
print("Log Queue Status:")
print(f"Processed {t_p} logs successfully.")
