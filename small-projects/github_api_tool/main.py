import json
from collections import defaultdict
from pathlib import Path
import requests

try:
    url = "https://api.github.com/users/sarik-khnn/repos?direction=asc"
    response = requests.get(url)
    response.raise_for_status()
    data = response.json()

    refined_data = defaultdict(list)

    for item in data:
        repo = item["name"]
        lang = item.get("language")
        if lang is None:
            lang = "No language"
        refined_data[lang].append(repo)

    p = Path("MyData")
    p.mkdir(exist_ok=True)
    p = p / "sarik.json"

    p.touch(exist_ok=True)
    with p.open("w") as f:
        json.dump(refined_data, f, indent=4)
    print("data written successfully !")
except requests.exceptions.RequestException as err:
    print(f"{err}")