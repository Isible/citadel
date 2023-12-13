# Rules

- When converting from a &str -> String always use to_owned() || String::from(...) || into instead of to_string() to avoid confusion

- Try avoid writing macros. 1. They are hard to debug, 2. Hard to maintain 3. The benefit of "clean-code" they provide comes with the big disadvantages (1. && 2.)
