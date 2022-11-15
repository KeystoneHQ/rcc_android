mod java_glue;
mod pb;
mod processors;

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
                self.sign(command.request_id, params)
            }
            Some(RequestData::GetRsaPublicKeyRequest(params)) => {
                self.get_rsa_public_key(command.request_id, params)
            }
            Some(RequestData::BlockChainRequest(params)) => {
                self.parse(command.request_id, params)
            }
            None => {
                CommandResponse::error(command.request_id, "Request is not supported".to_string())
            }
        }
    }

    fn parse(&self, request_id: u32, params: BlockChainRequest) -> CommandResponse {
        match processors::block_chain::process(params) {
            Ok(data) =>
                CommandResponse::success(request_id, data),
            Err(e) =>
                CommandResponse::error(request_id, e.to_string())
        }
    }

    fn sign(&self, request_id: u32, params: SignRequest) -> CommandResponse {
        match processors::sign_request::process(params) {
            Ok(data) =>
                CommandResponse::success(request_id, data),
            Err(e) =>
                CommandResponse::error(request_id, e.to_string())
        }
    }

    fn get_rsa_public_key(&self, request_id: u32, params: GetRsaPublicKeyRequest) -> CommandResponse {
        match processors::get_rsa_public_key_request::process(params) {
            Ok(data) =>
                CommandResponse::success(request_id, data),
            Err(e) =>
                CommandResponse::error(request_id, e.to_string())
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
