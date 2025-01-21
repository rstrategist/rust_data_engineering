# Rust AWS Step Functions Pipeline

The purpose of this module is to build step functions from AWS using Rust. Step functions are very powerful serverless workflows that can be used to orchestrate complex tasks. These tasks can be chained together, where the input of one lambda function is used as the input to another lambda function. By using these principles one can create very complex workflows.

We create new marco polo lambda by running:

```
cargo lambda new rust-marco
```

Then build, deploy and invoke:

```
make release
make deploy
make invoke
```

You may get an error if Zig is not installed. cargo-lambda uses Zig and cargo-zigbuild to compile the code for the right architecture. I'm working in GitHub codespaces so I need to install the vscode-zig extension (installs zig and zls) and put the zig install location in PATH:
```
echo 'PATH="$PATH:/home/codespace/.vscode-remote/data/User/globalStorage/ziglang.vscode-zig/zig_install"' >>~/.profile
```
After stopping and restarting the codespace, need to do a source ~/.profile to make sure zig stays in PATH.

 You can install cargo-zigbuild by running:
```
cargo install --locked cargo-zigbuild
```

Then we create a new rust polo lambda by running:

```
cargo lambda new rust-polo
```

![alt text](image.png)
