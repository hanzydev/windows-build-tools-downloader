use is_elevated::is_elevated;
use std::io::{self, Write};

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn exit() {
    println!("Exiting...");
    std::thread::sleep(std::time::Duration::from_secs(2));
    std::process::exit(0);
}

fn main() {
    if !is_elevated() {
        println!("Please run this program as administrator.");
        exit();
    }

    println!("This program will install the Node.js build tools for you.");
    println!("Note: Setup may take a short/long time depending on your internet speed.");

    print!("Do you want to install? [y/n]: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    if choice.trim() == "y" {
        let install_script = "powershell.exe Start-Process powershel -InputFormat None -ExecutionPolicy Bypass -NoProfile -Verb runAs -Command [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1')); choco upgrade -y python visualstudio2019-workload-vctools; Exit";

        let mut child = std::process::Command::new("powershell.exe")
            .arg("-Command")
            .arg(install_script)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        std::io::copy(&mut child.stdout.take().unwrap(), &mut std::io::stdout()).unwrap();

        clear();
        println!("Setup complete! Please restart your computer to apply changes.");
        exit();
    } else {
        clear();
        exit();
    }
}
