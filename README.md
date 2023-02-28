
### nu-cmd-lib

This introduces a new crate called nu-cmd-lib which consists of

* env
* system
* Table

The primary motiviation for this crate is to be able to run externals...

Run externals has a dependency on crate::Table

This will enable the tests in nu-cmd-lang to run and so they will live in this crate...
