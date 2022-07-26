mod java_glue;
mod pb;

use pb::abi::command_request::RequestData;
use pb::abi::*;
use prost::Message;
use std::io::Cursor;

use rcc_signer::{Signer, SigningAlgorithm};

pub use crate::java_glue::*;
use rifgen::rifgen_attr::*;

pub struct RCC;

impl RCC {
    #[generate_interface(constructor)]
    fn new() -> RCC {
        RCC {}
    }

    fn excute(&mut self, command: CommandRequest) -> CommandResponse {
        match command.request_data {
            Some(RequestData::SignRequest(params)) => {
                let signer = Signer::new_with_se(params.port_name);
                let sign_data = match hex::decode(&params.data) {
                    Ok(data) => data,
                    Err(_) => {
                        return CommandResponse::error(
                            command.request_id,
                            "sign data decode failed".to_string(),
                        )
                    }
                };

                let algo = match params.algo {
                    0 => SigningAlgorithm::Secp256k1,
                    _ => {
                        return CommandResponse::error(
                            command.request_id,
                            "Algo is not supported".to_string(),
                        )
                    }
                };

                match signer.sign_data(
                    params.seed_id as u8,
                    params.password,
                    sign_data,
                    algo,
                    params.derivation_path,
                ) {
                    Ok(signature) => {
                        return CommandResponse::success(command.request_id, hex::encode(signature))
                    }
                    Err(err) => return CommandResponse::error(command.request_id, err.to_string()),
                }
            }
            None => {
                CommandResponse::error(command.request_id, "Request is not supported".to_string())
            }
        }
    }

    fn deserialize_to_string(&self, command_respones: CommandResponse) -> String {
        let mut buf = Vec::new();
        buf.reserve(command_respones.encoded_len());

        command_respones.encode(&mut buf);
        hex::encode(buf)
    }

    #[generate_interface]
    pub fn process_command(&mut self, command_string: String) -> String {
        let buf: Vec<u8>;
        match hex::decode(&command_string) {
            Ok(data) => buf = data,
            Err(_) => {
                let cp = CommandResponse::error(
                    0,
                    "invaild command, decode protobuf hex error".to_string(),
                );
                return self.deserialize_to_string(cp);
            }
        }

        let cmd: CommandRequest;

        match CommandRequest::decode(&mut Cursor::new(&buf)) {
            Ok(command) => cmd = command,
            Err(_) => {
                let cp =
                    CommandResponse::error(0, "invaild command, decode protobuf error".to_string());
                return self.deserialize_to_string(cp);
            }
        }
        let cp = self.excute(cmd);
        self.deserialize_to_string(cp)
    }
}