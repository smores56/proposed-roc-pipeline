Proposed new Roc compiler pipeline
==================================

In an effort to align our plans for implementing the new compiler phases [including the lambda set fixing stages](https://github.com/roc-lang/rfcs/blob/ayaz/compile-with-lambda-sets/0102-compiling-lambda-sets.md#quick-view), this repo contains rough IR definitions and supplemental data structures for compiling the build phase (excluding codegen).

There are some simplifications made like using globally allocated data instead of arena-allocated data that make understanding this repo easier, those can be made more correct/performant during implementation.
