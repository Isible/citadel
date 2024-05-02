import sys

from shared import runCmd

# TODO: Add remaining tests

test_cmds = {
    "api": "cargo test --package api --lib -- tests --nocapture",
    "frontend": "cargo test --package frontend --lib -- tests --nocapture",
    "middleend": "cargo test --package middleend --lib -- tests --nocapture",
    "backend": "cargo test --package backend --lib -- tests --nocapture",
    "test-lang": "cargo test --package test-lang --bin test-lang -- tests --nocapture",
}

def runTest(selection: str):
    match selection:
        case "all":
            for cmd in test_cmds.values():
                runCmd(cmd)
        case _:
            runCmd(test_cmds[selection])


print("What tests should be ran?")
print("""Valid:
- `test-lang` - language for testing citadel
- `api` - citadel api for devs
- `frontend` - the compiler frontend (IR-gen)
- `middleend` - the compiler middleend (Optimizer)
- `backend` - the compiler backend (Machine code gen)
- `all` - run tests for all subprojects""")
selection = sys.argv[1]

runTest(selection)