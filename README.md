# Lua Linker
## Brief overview
As the description says, this is a lightweight lua linker made for merging code from multiple files into one. Useful for mod/hack development for roblox games where you can typically only inject a single script.

## Usage
Open the latest build [here](https://github.com/DiSaber/lua_linker/releases/latest) and download the `lua_linker.exe` file. Place it in a directory of your choice and double click to run.

The `main.lua` file is where the program starts merging code. You can include other `.lua` files inside the same directory by writing `-- include filename.lua` in the `main.lua` file. The program will grab the contents of any included files and drop them into the main file. The merged code will be placed into the `final.lua` file.

## Example
main.lua
```
-- include linkTest.lua

print("Hello from the main.lua file")
-- You can use the main file for staging and move other code into the included files
```

linkTest.lua
```
print("Hello from the linkTest.lua file")
-- You can write extra functions, variables, and anything else in the other files
```

and lastly the resulting final.lua file after the program is run
```
print("Hello from the linkTest.lua file")
-- You can write extra functions, variables, and anything else in the other files

print("Hello from the main.lua file")
-- You can use the main file for staging and move other code into the included files
```
