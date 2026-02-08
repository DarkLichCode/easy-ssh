// Public SSH test server provided by Rebex (https://test.rebex.net)
use easy_ssh::{SSHBuilder, AuthMethod};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	let ssh = SSHBuilder::new("test.rebex.net")
		.auth(AuthMethod::Password {
			username : "demo".into(),
			password : "password".into(),
		})
		.connect()?;

	println!("{}", ssh.exec("ls -la")?);

	let (result, status) = ssh.exec_with_status("whoami")?;
	println!("{}  {}", status, result);

	Ok(())
}
