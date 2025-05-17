In the `to_string()` method of the `Printer` structure, the current implementation attempts to abstract the display source using the `content` parameter of type `T`, which is constrained by the `SearchIssue` and `SearchCommit` traits.

**Option 1: Keep the abstraction**

* This approach allows for greater flexibility, as it allows the `Printer` to handle different types of data sources that implement the `SearchIssue` and `SearchCommit` traits.
* However, it requires the user to provide a custom implementation of these traits for their specific data source.

**Option 2: Specify the values directly**

* This approach is simpler and requires less code, as it avoids the need for custom traits.
* However, it is less flexible, as it is only suitable for working with the `Project` type.

**Recommendation:**

If the `Printer` is only ever going to be used with the `Project` type, it is acceptable to specify the values directly instead of using the abstraction. This simplifies the code and avoids unnecessary complexity.

**Suggested Code Update:**

```src/printer.rs
pub fn to_string(&self, project: &Project) -> String {
    // ...
}
```

This approach removes the `content` parameter and directly uses the `Project` type as the input.