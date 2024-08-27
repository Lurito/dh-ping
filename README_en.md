# DH-Ping

DH-Ping is a command-line tool for testing the connectivity of Dread Hunger servers.

This tool is developed using the Rust programming language, and contributions via Pull Requests are welcome.

For more documentation, please see:
- [User Manual](doc/Manual_en.md)
- [简体中文文档](README.md)
- [简体中文用户手册](doc/Manual.md)

## Usage Help

For non-developer users, please refer to the [User Manual](doc/Manual_en.md).

``` plaintext
Usage: dh-ping <IP:port>

Function:
  Sends a Dread Hunger UDP handshake packet to the specified
  <IP:port> to check server connectivity.

Options:
  -?, --help, help          Display this help information and exit
  -v, --version, version    Display the program version and exit

Description:
  If no <IP:port> is provided, the program will enter interactive mode.
  In this mode, you can repeatedly enter <IP:port> to check the
  connectivity of multiple server URIs.
```

## Program Features

DH-Ping sends a Dread Hunger UDP handshake packet to the specified URI. If a response is received from the server, it generally means that the server can be connected to normally.

DH-Ping has two modes: command-line mode and interactive mode. Entering `<IP:port>` as a command-line argument is command-line mode, and not entering it will enter interactive mode.

DH-Ping detects the system language (detection methods include the `LANG` and `LC_ALL` environment variables, and the Win32 `GetUserDefaultUILanguage` API) and automatically switches the display language.

## Open Source License

This project is open-sourced under the GNU LGPL-3.0 license. For details, see the [LICENSE](LICENSE.txt) file.

There are additional terms for this project: for the lurito/dh-ping project, if you have not made meaningful improvements to the program, you are not allowed to remove or modify the author's attribution, nor are you allowed to add authors to it.
