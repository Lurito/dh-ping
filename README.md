# DH-Ping

DH-Ping 是用于 Dread Hunger 服务器连通性测试的命令行工具。

本工具使用 Rust 语言开发，欢迎提交 Pull Request。

更多文档请见：
- [用户手册](doc/Manual.md)
- [English Document](README_en.md)
- [English User Manual](doc/Manual_en.md)

## 使用帮助

非开发者的用户请参阅 [用户手册](doc/Manual.md)。

``` plaintext
用法: dh-ping <IP:端口>

功能:
  发送 Dread Hunger UDP 握手包到指定的 <IP:端口>，以检测服务器的连通性。

选项:
  -?, --help, help          显示此帮助信息并退出
  -v, --version, version    显示程序的版本信息并退出

说明:
  如果未提供 <IP:端口>，程序将进入交互性模式。
  在此模式下，您可以反复输入 <IP:端口> 以检测多个服务器 URI 的连通性。
```

## 程序特性

DH-Ping 会向指定的 URI 发送 Dread Hunger UDP 握手包。若能得到服务器的响应，通常即意味着能正常连接到该服务器。

DH-Ping 有两种模式：命令行模式和交互性模式。输入 `<IP:端口>` 作为命令行参数即为命令行模式，不输入则会进入交互性模式。

DH-Ping 会检测系统语言（检测方式包括 `LANG` 和 `LC_ALL` 环境变量，和 Win32 `GetUserDefaultUILanguage` API），并自动切换显示语言（支持中、英文）。

## 开源许可

本项目采用 GNU LGPL-3.0 许可证开放源代码，详见 [LICENSE](LICENSE.txt) 文件。

本项目在以上协议之上，存在补充条款：针对 lurito/dh-ping 项目，如果您未对程序做有意义的改进，您不得删除、修改程序中的作者署名，也不得向其中增添作者名。
