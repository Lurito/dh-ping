use std::env;
use std::net::UdpSocket;
use std::time::Duration;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use rustyline::{
    Helper,
    hint::Hinter,
    completion::Completer,
    highlight::Highlighter,
    validate::Validator,
};

#[cfg(target_os = "windows")]
use winapi::um::winnls::GetUserDefaultUILanguage;

#[cfg(target_os = "windows")]
use winapi::um::wincon::SetConsoleTitleW;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn detect_language() -> &'static str {
    let lang = env::var("LANG").unwrap_or_default();
    let lc_all = env::var("LC_ALL").unwrap_or_default();

    if lang.starts_with("zh") || lc_all.starts_with("zh") {
        "zh"
    } else if !lang.is_empty() || !lc_all.is_empty() {
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

#[cfg(target_os = "windows")]
fn set_console_title(language: &str) {
    let title = match language {
        "zh" => "DH-Ping - Dread Hunger 服务器连通性测试工具",
        _ => "DH-Ping - Dread Hunger Server Connectivity Tool",
    };
    
    // Convert to UTF-16 for SetConsoleTitleW
    let wide_title: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    
    unsafe {
        SetConsoleTitleW(wide_title.as_ptr());
    }
}

#[cfg(not(target_os = "windows"))]
fn set_console_title(_language: &str) {
    // No-op on non-Windows platforms
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
    println!("Dread Hunger Ping Tool - v{} (2025-08-27)", built_info::PKG_VERSION);
    match language {
        "zh" => {
            println!("爱佐 (c) 2024-2025，根据 GNU 宽通用公共许可证 (LGPL) 授权。");
            println!("开源项目链接: {project_repo}");
        }
        _ => {
            println!("Ayrzo (c) 2024-2025. Licensed under the GNU Lesser General Public License.");
            println!("Project Repository: {project_repo}");
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
                "zh" => eprintln!("绑定 socket 失败: {e}"),
                _ => eprintln!("Failed to bind socket: {e}"),
            }

            stdout.reset().unwrap();
            io::stdout().flush().unwrap();

            std::process::exit(1);
        }
    };

    match socket.send_to(&payload, destination) {
        Ok(_) => {
            match language {
                "zh" => println!("数据已发往 {destination}"),
                _ => println!("Data sent to {destination}"),
            }
        }
        Err(e) => {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
            io::stdout().flush().unwrap();

            match language {
                "zh" => eprint!("数据发送失败: {e}"),
                _ => eprint!("Failed to send data: {e}"),
            }

            stdout.reset().unwrap();
            io::stdout().flush().unwrap();

            return;
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
                        print!("  {i:08x}: ");
                    }
                    print!("{byte:02x}");
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

    // Initialize rustyline helper for correctly set user input to yellow
    struct InputHelper;
    impl Helper for InputHelper {}
    impl Completer for InputHelper { type Candidate = String; }
    impl Hinter for InputHelper { type Hint = String; }
    impl Validator for InputHelper {}
    impl Highlighter for InputHelper {
        fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
            format!("\x1b[33m{line}\x1b[0m").into()
        }
        fn highlight_char(&self, _line: &str, _pos: usize, _forced: bool) -> bool {
            true
        }
    }

    let mut rl = rustyline::Editor::<InputHelper, rustyline::history::DefaultHistory>::new().unwrap();
    rl.set_helper(Some(InputHelper));

    // Set Ctrl + C/D handler
    ctrlc::set_handler(move || {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.reset().unwrap();
        io::stdout().flush().unwrap();
        std::process::exit(0);
    }).unwrap_or_else(|_| { panic!("{}", match language {
        "zh" => "初始化 SIGINT 信号处理逻辑失败，程序将退出。",
        _ => "Failed to initialize SIGINT signal processing logic, the program will exit.",
    }.to_string()) });

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
        let readline = rl.readline(prompt); // User input is set to yellow in helper

        stdout.reset().unwrap();
        io::stdout().flush().unwrap();

        match readline {
            Ok(line) => {
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(input);

                match input {
                    "exit" => {
                        std::process::exit(0);
                    },
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
            },
            Err(rustyline::error::ReadlineError::Interrupted) => {
                // Ctrl+C
                std::process::exit(0);
            },
            Err(rustyline::error::ReadlineError::Eof) => {
                // Ctrl+D
                std::process::exit(0);
            },
            Err(_) => {
                continue;
            }
        }
    }
}

fn main() {
    let language = detect_language();
    
    // Set console title on Windows
    set_console_title(language);
    
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
        println!();
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
