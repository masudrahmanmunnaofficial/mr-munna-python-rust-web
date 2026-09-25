"""Deterministic static reference data for the GitHub Pages build."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def build():
    output = ROOT / "static" / "data"
    output.mkdir(parents=True, exist_ok=True)
    data = {
        "version": 1,
        "country": "Bangladesh",
        "locations": [
            {"name": "Dhaka", "latitude": 23.8103, "longitude": 90.4125},
            {"name": "Chattogram", "latitude": 22.3569, "longitude": 91.7832},
            {"name": "Sylhet", "latitude": 24.8949, "longitude": 91.8687},
            {"name": "Rajshahi", "latitude": 24.3745, "longitude": 88.6042},
            {"name": "Khulna", "latitude": 22.8456, "longitude": 89.5403},
        ],
        "timezone": "Asia/Dhaka",
        "prayer_methods": [{"id": 1, "name": "Karachi"}, {"id": 2, "name": "ISNA"}],
    }
    path = output / "bangladesh.json"
    path.write_text(json.dumps(data, ensure_ascii=False, sort_keys=True, indent=2) + "\n")
    return path


if __name__ == "__main__":
    print(build())
