use std::io::Read;
use ssh2::Session;

use crate::error::SSHError;

pub struct SSHClient {
	_session: Session,
}

impl SSHClient {
	pub(crate) fn new(session: Session) -> Self {
		Self { _session: session }
	}

	pub fn exec(&self, cmd: &str) -> Result<String, SSHError> {
		let mut channel = self._session.channel_session()?;
		channel.exec(cmd)?;

		let mut output = String::new();
		channel.read_to_string(&mut output)?;
		channel.wait_close()?;

		Ok(output)
	}

	pub fn exec_with_status(&self, cmd: &str) -> Result<(String, i32), SSHError> {
		let mut channel = self._session.channel_session()?;
		channel.exec(cmd)?;

		let mut output = String::new();
		channel.read_to_string(&mut output)?;
		channel.wait_close()?;

		let exit_status = channel.exit_status()?;
		Ok((output, exit_status))
	}
}