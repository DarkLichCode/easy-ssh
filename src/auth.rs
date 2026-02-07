

pub enum AuthMethod {
	Password {
		username: String,
		password: String
	},
	Key {
		username: String,
		private_key: String,
		passphrase: Option<String>,
	},
}