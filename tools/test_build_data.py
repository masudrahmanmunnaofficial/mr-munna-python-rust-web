import json
import unittest
from build_data import build


class BuildDataTest(unittest.TestCase):
    def test_deterministic_locations(self):
        path = build()
        first = path.read_bytes()
        self.assertEqual(build().read_bytes(), first)
        data = json.loads(first)
        self.assertEqual(data["timezone"], "Asia/Dhaka")
        self.assertTrue(all(-90 <= c["latitude"] <= 90 for c in data["locations"]))


if __name__ == "__main__":
    unittest.main()
