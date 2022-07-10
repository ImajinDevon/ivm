<img src="https://repository-images.githubusercontent.com/505921096/e5f2b3f3-8939-4f66-bf8c-3fa326d04a4d" width="200" align="left" style="margin-right: 20px">

# ivm

`ivm` is an experimental, well-documented and expansion-ready virtual machine written in Rust.

`ivm` also provides a fairly medium level `Instruction` wrapper, which makes porting your language to `ivm` easier
than imaginable.

# Is it production ready?
No. As of core version 0.1.1-SNAPSHOT, ivm is still rapidly refactoring and changing its infrastructure.
ivm may not currently include enough operations for advanced software.

# Modules

| Module Name |    Version     | Since (core version) |                    Functionality                    |                        TODO                        |
|:-----------:|:--------------:|:--------------------:|:---------------------------------------------------:|:--------------------------------------------------:|
|  ivm-core   | 0.1.1-SNAPSHOT |        always        |    Will provide a CLI for executing ivm binaries    |                      Add CLI                       |
|   ivm-vm    |     0.1.1      |        always        | The virtual machine for executing ivm instructions. | Look into possible optimizations and code cleanup. |
| ivm-compile |     0.1.0      |        always        |                  The ivm compiler.                  |                        n/a                         |
