
## Cit 

### Simple single file version control tool
The tool is designed to be simple and easy to use. as opposed to Git, this tool does not mean to track changes in a file, but rather to keep as many versions as needed.

for example instead of having:
```
configs/
    config.json
    config_v2.json
    config_forDevelopment.json
    config_user1.json
    config_user2.json
```
you can have:
```
configs/
    config.json
```
and switch between them when needed.

#### Requirements & Installation:
- rust
- cargo

```
#install
make install

#uninstall
make uninstall
```

#### Usage:
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

# Remove a specific version
cit [filename] --remove [version]
```

Vscode extension:  TODO
