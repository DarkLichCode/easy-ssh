# 🚀easy-ssh
[![Crates.io](https://img.shields.io/crates/v/easy-ssh.svg)](https://crates.io/crates/easy-ssh) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/DarkLichCode/easy-ssh/blob/main/LICENSE)

A small, opinionated SSH client library built on top of ssh2.

This crate focuses on:
- Explicit configuration
- Clear error reporting
- Minimal abstractions over ssh2
- Non-interactive SSH use cases (exec-first)

It is designed as a building block for system tools, automation, and future async integrations.

## ✨Features

- **🛠️Fluent API :** Use SSHBuilder to easily configure connections.
- **🔑 Smart Auth :** Supports password authentication and public key (PEM/RSA) authentication.
- **🛡️ Fail-Fast Validation :** Pre-verify the private key format before connection to avoid obscure errors caused by libssh2's lack of support for the OpenSSH format.
- **🔍 Clear Errors :** Provide actionable error prompts with repair suggestions based on thiserror.

## 📦 Installation

This is a **synchronous (blocking)** library. For async support, stay tuned for our [**Roadmap**](#🗺️Roadmap)

**Cargo.toml**

```toml
[dependencies]
easy-ssh = "0.1"
```

## ⌨️ Quick Start

**Password Authentication**

```rust
use easy_ssh::{SSHBuilder, AuthMethod};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let ssh = SSHBuilder::new("host")
		.auth(AuthMethod::Password {
			username : "username".into(),
			password : "password".into(),
		})
		.connect()?;

	println!("{}", ssh.exec("ls -la")?);

	let (result, status) = ssh.exec_with_status("whoami")?;
	println!("{}  {}", status, result);

	Ok(())
}

```

**Public Key Authentication**

```rust
use easy_ssh::{SSHBuilder, AuthMethod};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let ssh = SSHBuilder::new("host")
		.auth(AuthMethod::Key {
			username : "username".into(),
			private_key : "private_key full path".into(),
			passphrase : None,
		})
		.connect()?;

	println!("{}", ssh.exec("ls -la")?);

	let (result, status) = ssh.exec_with_status("whoami")?;
	println!("{}  {}", status, result);

	Ok(())
}
```

## ⚠️Private Key Format

This library uses `libssh2`, which does **not** support OpenSSH private key format.

Generate a compatible key using:

```shell
ssh-keygen -t rsa -b 4096 -m PEM
```

## 🗺️Roadmap

✅ Password Authentication

✅ Public Key Authentication

✅ Execute shell command

⬜ Async backend (tokio + openssh)

⬜ Interactive shell support

⬜ SFTP Support

⬜ Connection pooling

## 📜License

MIT [LICENSE](https://github.com/DarkLichCode/easy-ssh/blob/v0.1/LICENSE)
