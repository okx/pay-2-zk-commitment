use std::time::Duration;

use reqwest::{blocking::Client, header};
use serde::{Deserialize, Serialize};

pub struct RapidSnark {
    prover_url: String,
    client: Client,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Groth16Proof {
    pub pi_a: [String; 3],
    pub pi_b: [[String; 2]; 3],
    pub pi_c: [String; 3],
    pub protocol: String,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct Groth16ProofWithPublicData {
    pub proof: Groth16Proof,
    pub public_data: Vec<String>,
}

impl From<StatusResponse> for Groth16ProofWithPublicData {
    fn from(input: StatusResponse) -> Self {
        let parsed_proof: Result<Groth16Proof, serde_json::Error> =
            serde_json::from_str(&input.proof.unwrap());
        let parsed_pub_data: Result<Vec<String>, serde_json::Error> =
            serde_json::from_str(&input.pubData.unwrap());

        Groth16ProofWithPublicData {
            proof: parsed_proof.unwrap(),
            public_data: parsed_pub_data.unwrap(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct StatusResponse {
    pub proof: Option<String>,
    pub pubData: Option<String>,
    pub status: String,
}

impl RapidSnark {
    pub fn new(prover_url: &str) -> Self {
        let mut default_header = header::HeaderMap::new();
        default_header.insert("Content-Type", header::HeaderValue::from_static("application/json"));
        default_header.insert("Accept", header::HeaderValue::from_static("application/json"));
        let client = reqwest::blocking::ClientBuilder::new()
            // .http2_keep_alive_while_idle(true)
            .default_headers(default_header)
            .build()
            .expect("unable to build http client");

        Self { client, prover_url: prover_url.to_string() }
    }

    pub fn get_status(&self) -> StatusResponse {
        let _response = self.client.get(format!("{}/status", &self.prover_url)).send().unwrap();
        let ret = _response.json::<StatusResponse>().unwrap();
        ret
    }

    pub fn request_proof_sync(&self, proof_json_string: String) -> StatusResponse {
        let _response = self
            .client
            .post(format!("{}/input/plonky2", &self.prover_url))
            .body(proof_json_string)
            .send();
        loop {
            let status_response = self.get_status();
            if status_response.status != "busy" {
                break status_response;
            }
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}

use num_bigint::BigInt;
// use num_traits::One;
use std::str::FromStr;
pub fn num_str_to_hex_str_u256(input: &str) -> String {
    let a = BigInt::from_str(input).expect("Invalid number string");
    let hex_string = a.to_str_radix(16);
    // Desired length for the hexadecimal string with zero padding
    let desired_length = 64;
    // Add zero padding
    let padded_hex_string = format!("0x{:0>width$}", hex_string, width = desired_length);
    padded_hex_string
}

#[cfg(test)]
mod test {
    use super::num_str_to_hex_str_u256;

    #[test]
    fn test_num_str_to_hex_str_u256() {
        assert_eq!(
            num_str_to_hex_str_u256(
                "4895406746137314371474712892123577126446751675251153276348512357056586750446"
            ),
            "0x0ad2b3b820ff14b318cd6a25e3a47935d2456d4c157a3b3eed2d052c457499ee"
        );
        assert_eq!(
            num_str_to_hex_str_u256(
                "4576621004412920114621308321746549921874306513740604599183379873099492536090"
            ),
            "0x0a1e4693ca11f48d299a341086ee13352925e6a7c6c61b68b3aa012f8fd6af1a"
        );
        assert_eq!(
            num_str_to_hex_str_u256(
                "14164655910904992320028415100632011246092840499993364460799609235949144057071"
            ),
            "0x1f50e9404560156854144cfd6a6fb509343bb5b836a612e05b40466d39bd24ef"
        );

        assert_eq!(
            num_str_to_hex_str_u256("4364051170455369011"),
            "0x0000000000000000000000000000000000000000000000003c90396af49eb533"
        );
        assert_eq!(
            num_str_to_hex_str_u256("4390174180335613484"),
            "0x0000000000000000000000000000000000000000000000003ced082828fef62c"
        )
    }
}
