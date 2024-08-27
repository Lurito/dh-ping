# DH-Ping User Manual

DH-Ping is a small tool used to test whether the Dread Hunger server can be connected.

**Important Notice: If you test a remote server, the server admin may see your IP address:**

```plaintext
LogNet: NotifyAcceptingConnection accepted from: 123.45.67.89:43210
```

## Download and Launch

First, you need to download the latest version of DH-Ping from the [Releases page](https://github.com/Lurito/dh-ping/releases).

You need to confirm the operating system of the **computer that will run the program**:
- If it is a Windows system, download the one with the `windows-amd64` suffix;
- If it is a Linux system, download the one with the `linux-amd64` suffix.

### Launching on Windows

After downloading, you need to extract the `.zip` file, which will give you a `dh-ping.exe` file.

Double-click the program to launch it, and you will enter interactive mode.

### Launching on Linux

After downloading, use the `cd` command to navigate to the directory where the `.zip` file is located. Then execute `unzip dh-ping_linux-amd64.zip` to extract the file, which will give you a `dh-ping` file.

If you encounter an error because `unzip` is not installed, please search online for how to install it.

After extraction, execute `chmod +x ./dh-ping` to grant the program execution permissions. Then execute `./dh-ping` to launch the program and enter interactive mode.

### Command Line Mode (for advanced users only)

Execute `./dh-ping <IP:Port>` to run the program directly in the command line. For more information, refer to `./dh-ping help`.

## Starting the Test

### How to Determine Connectivity

First, we need to understand which test results indicate that the server is properly connected and which ones indicate that it is not.

For example, in interactive mode, after entering `IP:Port` in the command line and pressing Enter, the program will display some information (the data in the example has been processed):

```plaintext
DH-Ping > 127.0.0.1:7777
Data sent to 127.0.0.1:7777
Received data:
  00000000: 0000 1111 2222 3333 4444 5555 6666 7777 
  00000010: 8888 9999 aaaa bbbb cccc dddd ee
```

In the above example, we can see that the program received data from the server. Generally speaking, this indicates that the server can be connected.

```plaintext
DH-Ping > 127.0.0.1:8888
Data sent to 127.0.0.1:8888
[No data received]
```

In this example, the program did not receive any data, indicating that the program cannot communicate with the server. Generally speaking, this means the game will also be unable to connect to this server.

## Troubleshooting Server Connection Issues

If you find that a server cannot be connected, you can follow this procedure to troubleshoot.

First, we need to know which port the server program is running on. We can find the following information on the server program (which is the "black window with scrolling text"):

```plaintext
LogNet: Created socket for bind address: 0.0.0.0 on port 7777
```

This means our server is running on port 7777.

You need to manually find the port number on the server program. If your port is not 7777, you need to adjust the port number accordingly when following the instructions.

Then, run the program **<u>on the server</u>** and test in the following order:
1. Test `127.0.0.1:7777`;
2. Test `<Internal IP>:7777`;
3. Test `<Public IP>:7777`.

Your internal IP address and public IP address can usually be found on the server provider's website panel (e.g., Alibaba Cloud). `<Public IP>:<Port>` is the string you enter in the game to join the server.

If you cannot join the server in the game, it usually means that at least one of the tests will not receive data. The solutions are as follows:
1. If you did not receive data when testing `127.0.0.1:7777`, you need to check:
    - Are you running the testing program **<u>on the server</u>**?
    - Is the server program running normally? You can usually see something like `(Engine Initialization) Total time: 4.36 seconds` in the server program
    - Are you sure you followed the instructions above and found the correct port number?
    - In the "Find Port Number" step, does it show `bind address: 0.0.0.0`?
    - If you're using a Windows system, check if the server program's top-left corner displays the word "Select". If it does, click on the server program window and press Ctrl+C on your keyboard, then try testing again.
2. If you did not receive data when testing `<Internal IP>:7777`, you need to check:
    - Is the internal IP address entered correctly?
    - Is the server firewall turned off? You can search online for "How to turn off the firewall on [your system]"; for more experienced users, it's recommended not to turn off the firewall but to set rules in the firewall to allow the connection.
    - If port forwarding is set up, is it configured correctly?
    - In the "Find Port Number" step, does it show `bind address: 0.0.0.0`?
    - On the server provider’s panel, are the firewall or security group settings configured correctly?
3. If you did not receive data when testing `<Public IP>:7777`, you need to check:
    - Is the public IP address entered correctly?
    - On the server provider's panel, are the firewall or security group settings configured correctly? Is public bandwidth properly enabled?
    - Is the network status of the server provider normal?
    - Perhaps you can consult the server provider's customer support for this issue.

## Sharing the Software

You are free to share this program with anyone without modification. You can also modify and distribute the program under the GNU LGPL-3.0 license.

However, this project has additional terms: for the `lurito/dh-ping` project, if you have not made meaningful improvements to the program, you are not allowed to remove or modify the author's attribution, nor are you allowed to add authors to it.
