use thiserror::Error;

#[derive(Error, Debug)]
pub enum SSHError {
	#[error("IO Error: {0}")]
	IO(#[from] std::io::Error),

	#[error("Unsupported private key format: {detected}. {hint}")]
	UnsupportedPrivateKeyFormat {
		detected: String,
		hint: &'static str,
	},

	#[error("SSH Error: {0}")]
	SSH(#[from] ssh2::Error),

	#[error("Authentication Method Missing")]
	MissingAuth,

	#[error("Authentication failed")]
	AuthFailed,
}