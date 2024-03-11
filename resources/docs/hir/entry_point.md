# Entry points in hir

Every Citadel-IR program needs an entry point. This uses the entry

This entry point should then call your main function. However, it can do pretty much anything.

**Important**: [S2C] The last expression in the entry label will be used as the exit code, assuming it is in a valid number format (preferably an i32). If this is not the case, your program will return the exit code 0, meaning it was executed sucessfully.

**Tip**: Inline your main function if you do not plan to put any other logic into the _entry label.

**Example**:

```txt
entry {
    call %main()
}

@main() i32 {
    $exit_code i32 = l{0}
    ret exit_code
}
```
