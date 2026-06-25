def traffic_analyzer(
    server_name: str, *requests, timeout: int = 60, **settings
) -> dict:
    """claud traffic analyzer and billing engine"""
    tax_pct = settings.get("tax_pct", 0)
    discount_val = settings.get("discount_val", 0)
    mal_threshold = settings.get("mal_threshold", 0)
    processed = 0
    for req in requests:
        if req > mal_threshold:
            print(f"ALERT !, Dos Attack Detected At {req} MB ! Halting Processing.")
            print()
            break
        elif req == 0:
            continue
        print(f"Processing Request: {req} MB")
        processed = processed + req
    else:
        print("Server Run Completed Safely !")
        print()

    after_disc = processed - ((processed / 100) * discount_val)
    final = after_disc + ((after_disc / 100) * tax_pct)
    report = {
        "server": server_name,
        "timeout": timeout,
        "processed": processed,
        "final": final,
    }
    return report


result = traffic_analyzer(
    "Alpha-Node", 15, 20, 0, 45, 120, 10, tax_pct=10, discount_val=5, mal_threshold=100
)
server = result["server"]
timeout = result["timeout"]
processed = result["processed"]
final = result["final"]
print("[FINAL REPORT]")
print(f"Server: {server}")
print(f"Timeout set to: {timeout}s")
print(f"Valid traffic processed: {processed} MB")
print(f"Final Bill Amount: ${final}")
