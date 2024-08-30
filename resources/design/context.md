# Context

Sometimes the ir needs to store context about a specific struct or function. This is what context(annotation)s are for.

```chir
@ctx(inline)
func @foo($bar i32) void {
    ...
}
```
