#  vcargo
## _A submodule of cargo_
>    A "private" project but that can be useful to other users, it simply pre-processes functions before and after the [**cargo**](https://doc.rust-lang.org/cargo/) commands.
>
## Features

- Can execute functions before and after [**cargo**](https://doc.rust-lang.org/cargo/) commands in cli.
- Modification simple, with few lines of code you can add macros to your vcargo to improve your development.
- It is a good project for beginners to adapt on rust language and learn the cli commands.

## How to use
### Compilation and paths:
You can compile it with your own [**cargo**](https://doc.rust-lang.org/cargo/) and create an optimized build. After creating your executable it is necessary to add it to the environment path of your system, either inside the `target` folder of your project, or by moving it to another specific folder for applications that need to be in the system paths.

### Code modifications:
Its use is simple, and can be changed to your style. For example, a current additional module that when starting a project with the command `vcargo init`,  add files after initialization of project from a predetermined folder, so you can change this module changing the path of the constant `_CUSTOM_FILE_INIT` in the file `main.rs`, other than that, accessing the function`check_and_run_command`. where below the execution of the [**cargo**](https://doc.rust-lang.org/cargo/), or even prior to it, functions can be added ...


###  Current  modules:
- After init new project:  Copies files from a predetermined folder to this new project.

## License
MIT
