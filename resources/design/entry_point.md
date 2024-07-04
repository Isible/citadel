# Entry points in hir

Every Citadel-IR program needs an entry point. This uses the entry

This entry point should then call your main function. However, it can do pretty much anything.

**Important**: To successfully exit in citadel you need to use the exit keyword.

**Tip**: Inline your main function if you do not plan to put any other logic into the entry block.

**Example**:

```chir
entry {
    call %main()
}

func @main() i32 {
    $exit_code i32 = l{0:i32}
    ret exit_code
}
```
