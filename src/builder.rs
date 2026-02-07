use std::net::TcpStream;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::net::ToSocketAddrs;

use ssh2::Session;

use crate::auth::AuthMethod;
use crate::client::SSHClient;
use crate::error::SSHError;

pub struct SSHBuilder {
	host: String,
	port: u16,
	auth: Option<AuthMethod>,
	timeout: Option<Duration>,
}

impl SSHBuilder {
	pub fn new(host: impl Into<String>) -> Self {
		Self {
			host: host.into(),
			port: 22,
			auth: None,
			timeout: Some(Duration::from_secs(10)),
		}
	}

	pub fn port(mut self, port: u16) -> Self {
		self.port = port;
		self
	}

	pub fn auth(mut self, auth: AuthMethod) -> Self {
		self.auth = Some(auth);
		self
	}

	pub fn timeout(mut self, dur: Duration) -> Self {
		self.timeout = Some(dur);
		self
	}

	pub fn connect(self) -> Result<SSHClient, SSHError> {
		let auth = self.auth.ok_or(SSHError::MissingAuth)?;
		if let AuthMethod::Key { ref private_key, .. } = auth {
			detect_private_key_format(&std::path::Path::new(private_key))?;
		}

		let addr = (self.host.as_str(), self.port)
			.to_socket_addrs()?
			.next()
			.ok_or_else(|| SSHError::IO(std::io::Error::new(
				std::io::ErrorKind::Other,
				"Unable to resolve address",
			)))?;

		let tcp = if let Some(timeout) = self.timeout {
			TcpStream::connect_timeout(&addr, timeout)?
		} else {
			TcpStream::connect(addr)?
		};

		if let Some(timeout) = self.timeout {
			tcp.set_read_timeout(Some(timeout))?;
			tcp.set_write_timeout(Some(timeout))?;
		}

		let mut session = Session::new()?;
		session.set_tcp_stream(tcp);
		session.handshake()?;

		match auth {
			AuthMethod::Password {username, password} => {
				session.userauth_password(&username, &password)?;
			},
			AuthMethod::Key {username, private_key, passphrase} => {
				session.userauth_pubkey_file(&username, None, &std::path::Path::new(&private_key), passphrase.as_deref())?;
			}
		}

		if !session.authenticated() {
			return Err(SSHError::AuthFailed);
		}

		Ok(SSHClient::new(session))
	}
}

fn detect_private_key_format(path: impl AsRef<Path>) -> Result<(), SSHError> {
	let file = File::open(path)?;
	let mut reader = BufReader::new(file);

	let mut first_line = String::new();
	reader
		.read_line(&mut first_line)?;

	let line = first_line.trim();

	if line.contains("BEGIN RSA PRIVATE KEY") {
		Ok(())
	} else if line.contains("BEGIN PRIVATE KEY") {
		Ok(())
	} else if line.contains("BEGIN OPENSSH PRIVATE KEY") {
		Err(SSHError::UnsupportedPrivateKeyFormat {
			detected: "OpenSSH".into(),
			hint: "libssh2 does not support OpenSSH private key format. \
Generate a PEM key using: ssh-keygen -t rsa -b 4096 -m PEM",
		})
	} else {
		Err(SSHError::UnsupportedPrivateKeyFormat {
			detected: "Unknown".into(),
			hint: "Unrecognized private key format.",
		})
	}
}