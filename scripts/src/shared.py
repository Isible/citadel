import subprocess


def runCmd(cmd: str) -> None:
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    print(result.stdout)