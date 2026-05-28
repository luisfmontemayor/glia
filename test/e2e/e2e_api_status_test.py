import subprocess

def test_api_status():
    subprocess.run(["mise", "run", "api:status"], check=True)
