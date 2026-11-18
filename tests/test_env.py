from pathlib import Path

import tomllib


def test_profile_toml_exists_and_valid():
    root = Path(__file__).resolve().parent.parent
    profile_path = root / "profile.toml"
    assert profile_path.exists(), "profile.toml must exist in root directory"

    with profile_path.open("rb") as f:
        data = tomllib.load(f)

    assert "student" in data
    student = data["student"]
    for field in ["name", "nim", "class", "study_program", "institution"]:
        assert field in student, f"Missing required student field: {field}"
        assert isinstance(student[field], str)
        assert len(student[field].strip()) > 0

    assert "naming" in data
    assert "pattern" in data["naming"]
