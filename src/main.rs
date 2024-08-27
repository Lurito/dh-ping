use std::env;
use std::net::UdpSocket;
use std::time::Duration;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetUserDefaultUILanguage;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn detect_language() -> &'static str {
    let lang = env::var("LANG").unwrap_or_default();
    let lc_all = env::var("LC_ALL").unwrap_or_default();

    if lang.starts_with("zh") || lc_all.starts_with("zh") {
        "zh"
    } else if lang.len() > 0 || lc_all.len() > 0 {
        "en"
    } else {
        #[cfg(target_os = "windows")]
        {
            let lang_id = unsafe { GetUserDefaultUILanguage() };
            if lang_id == 0x0804 || lang_id == 0x1004 {
                // Simplified Chinese or Traditional Chinese
                return "zh";
            }
        }
        "en"
    }
}

fn print_help(language: &str) {
    match language {
        "zh" => {
            println!("用法: dh-ping <IP:端口>");
            println!();
            println!("功能:");
            println!("  发送 Dread Hunger UDP 握手包到指定的 <IP:端口>，以检测服务器的连通性。");
            println!();
            println!("选项:");
            println!("  -?, --help, help          显示此帮助信息并退出");
            println!("  -v, --version, version    显示程序的版本信息并退出");
            println!();
            println!("说明:");
            println!("  如果未提供 <IP:端口>，程序将进入交互性模式。");
            println!("  在此模式下，您可以反复输入 <IP:端口> 以检测多个服务器 URI 的连通性。");
        }
        _ => {
            println!("Usage: dh-ping <IP:port>");
            println!();
            println!("Function:");
            println!("  Sends a Dread Hunger UDP handshake packet to the specified");
            println!("  <IP:port> to check server connectivity.");
            println!();
            println!("Options:");
            println!("  -?, --help, help          Display this help information and exit");
            println!("  -v, --version, version    Display the program version and exit");
            println!();
            println!("Description:");
            println!("  If no <IP:port> is provided, the program will enter interactive mode.");
            println!("  In this mode, you can repeatedly enter <IP:port> to check the");
            println!("  connectivity of multiple server URIs.");
        }
    }
}

fn print_version(language: &str) {
    let project_repo = built_info::PKG_REPOSITORY;
    println!("Dread Hunger Ping Tool - v{} (2024-08-27)", built_info::PKG_VERSION);
    match language {
        "zh" => {
            println!("爱佐 (c) 2024，根据 GNU 宽通用公共许可证 (LGPL) 授权。");
            println!("开源项目链接: {}", project_repo);
        }
        _ => {
            println!("Ayrzo (c) 2024. Licensed under the GNU Lesser General Public License.");
            println!("Project Repository: {}", project_repo);
        }
    }
}

fn is_valid_ip_port(input: &str) -> bool {
    let parts: Vec<&str> = input.split(':').collect();
    if parts.len() != 2 {
        return false;
    }
    let ip = parts[0];
    let port = parts[1];

    if ip.parse::<std::net::IpAddr>().is_err() {
        return false;
    }

    if port.parse::<u16>().is_err() {
        return false;
    }

    true
}

fn send_and_receive(destination: &str, language: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let payload: [u8; 29] = [
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
    ];

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            io::stdout().flush().unwrap();
    
            match language {
                "zh" => eprintln!("绑定 socket 失败: {}", e),
                _ => eprintln!("Failed to bind socket: {}", e),
            }

            stdout.reset().unwrap();
            io::stdout().flush().unwrap();

            std::process::exit(1);
        }
    };

    match socket.send_to(&payload, &destination) {
        Ok(_) => {
            match language {
                "zh" => println!("数据已发送到 {}", destination),
                _ => println!("Data sent to {}", destination),
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            io::stdout().flush().unwrap();

            match language {
                "zh" => eprintln!("数据发送失败: {}", e),
                _ => eprintln!("Failed to send data: {}", e),
            }

            stdout.reset().unwrap();
            io::stdout().flush().unwrap();

            std::process::exit(1);
        }
    }

    socket.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
    let mut buffer = [0u8; 32];

    match socket.recv_from(&mut buffer) {
        Ok((size, _)) => {
            if size > 0 {
                match language {
                    "zh" => println!("收到数据:"),
                    _ => println!("Received data:"),
                }

                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
                io::stdout().flush().unwrap();

                for (i, byte) in buffer.iter().take(size).enumerate() {
                    if i % 16 == 0 {
                        print!("  {:08x}: ", i);
                    }
                    print!("{:02x}", byte);
                    if i % 2 == 1 { // Add a space after every two bytes
                        print!(" ");
                    }
                    if i % 16 == 15 && i != size - 1 {
                        println!();
                    }
                }

                stdout.reset().unwrap();
                io::stdout().flush().unwrap();
            } else {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap();
                io::stdout().flush().unwrap();

                match language {
                    "zh" => print!("[未收到任何数据]"),
                    _ => print!("[No data received]"),
                }

                stdout.reset().unwrap();
                io::stdout().flush().unwrap();
            }
        }
        Err(_) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta))).unwrap();
            io::stdout().flush().unwrap();

            match language {
                "zh" => print!("[未收到任何数据]"),
                _ => print!("[No data received]"),
            }
    
            stdout.reset().unwrap();
            io::stdout().flush().unwrap();
        }
    }
}

fn repl_mode(language: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    ctrlc::set_handler(move || {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.reset().unwrap();
        io::stdout().flush().unwrap();
        std::process::exit(0);
    }).expect(
        match language {
            "zh" => "初始化 SIGINT 信号处理逻辑失败，程序将退出。",
            _ => "Failed to initialize SIGINT signal processing logic, the program will exit.",
        }
    );

    // Title hint
    print_version(language);
    println!();
    match language {
        "zh" => {
            println!("输入 IP:端口 查询 Dread Hunger 服务器端口连通性，例如：127.0.0.1:7777");
            println!("输入 'exit' 退出");
        },
        _ => {
            println!("Enter IP:port to check connectivity of Dread Hunger server, e.g., 127.0.0.1:7777");
            println!("Enter 'exit' to quit");
        },
    }
    println!();

    // Main logic
    loop {
        let prompt = "DH-Ping > ";
        print!("{}", prompt);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        stdout.reset().unwrap();
        io::stdout().flush().unwrap();

        match input {
            "exit" => {
                std::process::exit(0);
            },
            // "help" => {
            //     print_help(language);
            //     continue;
            // },
            // "version" => {
            //     print_version(language);
            //     continue;
            // },
            _ => {
                if !is_valid_ip_port(input) {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
                    io::stdout().flush().unwrap();

                    match language {
                        "zh" => eprintln!("错误: 无效的 IP:端口 格式。正确示例：127.0.0.1:7777"),
                        _ => eprintln!("Error: Invalid IP:port format. Correct example: 127.0.0.1:7777"),
                    }

                    stdout.reset().unwrap();
                    io::stdout().flush().unwrap();

                    continue;
                }

                send_and_receive(input, language);
                println!();
            }
        }
    }
}

fn main() {
    let language = detect_language();
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let destination = &args[1];

        if destination == "help" || destination == "--help" || destination == "-?" {
            print_help(language);
            return;
        }

        if destination == "version" || destination == "--version" || destination == "-v" {
            print_version(language);
            return;
        }

        if !is_valid_ip_port(destination) {
            match language {
                "zh" => eprintln!("错误: 无效的 IP:端口 格式。执行 `dh-ping help` 获取更多帮助。"),
                _ => eprintln!("Error: Invalid IP:port format. Run `dh-ping help` for more help."),
            }
            print_help(language);
            std::process::exit(1);
        }

        send_and_receive(destination, language);
    } else if args.len() == 1 {
        repl_mode(language);
    } else {
        match language {
            "zh" => eprintln!("错误: 参数数量过多。执行 `dh-ping help` 获取更多帮助。"),
            _ => eprintln!("Error: Too many arguments. Run `dh-ping help` for more help."),
        }
        print_help(language);
        std::process::exit(1);
    }
}
