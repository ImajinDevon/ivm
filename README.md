<img src="https://repository-images.githubusercontent.com/505921096/e5f2b3f3-8939-4f66-bf8c-3fa326d04a4d" width="200" align="left">

# ivm
`ivm` is an experimental, well-documented and expansion-ready virtual machine written in Rust.

`ivm` also provides a fairly medium level `Instruction` wrapper, which makes porting your language to `ivm` easier
than imaginable.

&nbsp;

# Modules

| Module Name | Version | Since (core version) | Functionality | TODO |
|:---:|:---:|:---:|:---:|:---:|
| ivm-core | 0.1.0-SNAPSHOT | 0.1.0-SNAPSHOT | Provides utility methods for dependent crates, no functionality as of version 0.1.0-SNAPSHOT | Add proper CLI |
| ivm-vm | 0.0.1 | 0.1.0-SNAPSHOT | The virtual machine for executing ivm instructions. | Look into possible optimizations and VM configurations |
| ivm-common | 0.0.1 | 0.1.0-SNAPSHOT | Provides common methods to ivm modules. | n/a |
| ivm-compile | 0.0.1 | 0.1.0-SNAPSHOT | The ivm compiler. | Implement primitive types, more instructions |
