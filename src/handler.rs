use protocol::Request;
use protocol::Response;

use error::HandleResult;

pub trait SSHAgentHandler: Send + Sync {
	fn new() -> Self;
	fn identities(&self) -> HandleResult<Response>;
	fn sign_request(&self, pubkey: Vec<u8>, data: Vec<u8>, flags: u32) -> HandleResult<Response>;

	fn handle_request(&self, request: Request) -> HandleResult<Response> {
		match request {
			Request::RequestIdentities => {
				self.identities()
			}
			Request::SignRequest {ref pubkey_blob, ref data, ref flags} => {
				self.sign_request(pubkey_blob.clone(), data.clone(), flags.clone())
			}
			Request::Unknown => {
				Ok(Response::Failure)
			}

		}
	}
}

