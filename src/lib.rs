use crypto::poseidon::{PoseidonHasher, PoseidonSignature};
use heimdall_nft::NFTCredential;
use ledger_authority::{remote_ledger_authority::RemoteLedgerAuthority, LedgerAuthority};
use num_bigint::BigInt;
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

pub mod heimdall_nft_server_structs{
    use super::*;


    #[derive(Serialize, Deserialize)]
    pub struct NewParams {
        pub attributes: Vec<String>,
        pub id: u64,
        pub expiration: u128,
        pub credential_type: String,
        pub delegatable: bool,
        pub registry: String,
        pub sk_issuer: BigInt,
        pub owner: BigInt,
        pub trust_list_index: usize,
    }


    #[derive(Serialize, Deserialize)]
    pub struct TransferParams{
        pub nft: NFTCredential<PoseidonHasher, PoseidonSignature>,
        pub private_key: BigInt,
        pub fraction: [BigInt; 2],
        pub target_a: BigInt,
        pub target_b: BigInt,
    }


    #[derive(Serialize, Deserialize)]
    pub struct BurnParams{
        pub nft: NFTCredential<PoseidonHasher, PoseidonSignature>,
        pub private_key: BigInt
    }

    #[derive(Serialize, Deserialize)]
    pub struct AttachParams{
        pub nft: NFTCredential<PoseidonHasher, PoseidonSignature>,
        pub private_key: BigInt,
        pub target: NFTCredential<PoseidonHasher, PoseidonSignature>
    }

    #[derive(Serialize, Deserialize)]
    pub struct DetachParams{
        pub nft: NFTCredential<PoseidonHasher, PoseidonSignature>,
        pub private_key: BigInt,
        pub target: BigInt,
        pub path_nft_credential: Vec<NFTCredential<PoseidonHasher, PoseidonSignature>>
    }
}