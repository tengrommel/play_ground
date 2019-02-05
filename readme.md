# rust

vscode 调试文件

launch.json

```
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "play_ground",
            "program": "${workspaceRoot}/target/debug/play_ground",
            "args": [],
            "cwd": "${workspaceRoot}/target/debug/"
        }
    ]
}
```

rust编译命令 

    rustc main.rs

# PROGRAMMING A GUESSING GAME
