
## Cit 

#### Simple single file version control tool

requirements:
- rust
- cargo

```
#install
make install

#uninstall
make uninstall
```

Usage:
```
# Initialize a file for versioning (Creates a first "baseline" version)
cit [filename] --init

# Add a new version to the file (Creates a new version with the current content)
cit [filename] --add

# Commit the changes to the current version
cit [filename] --commit

# List all versions
cit [filename] --list

# Switch to a specific version
cit [filename] --switch [version]
```

Vscode extension:  TODO
