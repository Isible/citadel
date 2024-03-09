# Entry points in hir

Every Citadel-IR program needs an entry point. Right now this is hardcoded to be the "_entry" label. However this will probably be configurable in the future.

This entry point should then call your main function. However, it can do pretty much anything.

**Important**: The last expression in the entry label will be used as the exit code, assuming it is in a valid number format (preferably an i32). If this is not the case, your program will return the exit code 1, meaning it was not executed sucessfully.

**Tip**: Inline your main function if you do not plan to put any other logic into the _entry label.
