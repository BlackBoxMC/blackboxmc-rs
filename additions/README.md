This folder contains many files consisting of manually written code that gets injected into the generated code.

The format for this is as follows:

```
//[file_path]#[struct]
[code]
```

The struct part is optional, but if you put it in, your code will be appended to the relevant struct instead of just the bottom of the file.

Simply type the code into appropriately named files, and make sure the first line starts with two slashes and the path (relative to `generate.py`) where the code should be injected