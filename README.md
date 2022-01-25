# `bye-nm` :wave:

A trivial tool to get rid of `node_modules` by moving it to `$TMPDIR` instead of deleting it (which is must faster than doing an `rm -rf`) and then letting the OS clean it up later.
