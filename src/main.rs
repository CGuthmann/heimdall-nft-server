#[macro_use]
extern crate rocket;
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
}

use heimdall_nft_server_structs::*;

const LEDGER_URL: &str = "http://localhost:8000";
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/new", data = "<data>", format = "json")]
fn new(
    data: Json<NewParams>
) -> Result<Json<NFTCredential<PoseidonHasher, PoseidonSignature>>, Status> {
    let data = data.0;

    let mut ledger_authority = RemoteLedgerAuthority::new(LEDGER_URL.to_owned());
    
    let proof = ledger_authority.get_trust_list().generate_proof(data.trust_list_index).unwrap();

    let nft = NFTCredential::new(
        &data.attributes,
        data.id,
        data.expiration,
        &data.credential_type,
        data.delegatable,
        &data.registry,
        &data.sk_issuer,
        &data.owner,
        &mut ledger_authority,
        &proof,
    );

    match nft {
        Ok(nft) => Ok(Json(nft)),
        Err(_) => Err(Status::InternalServerError),
    }
}


#[post("/transfer_split", data = "<data>", format = "json")]
fn transfer_split(data: Json<TransferParams>) -> Result<Json<(
    NFTCredential<PoseidonHasher, PoseidonSignature>,
    NFTCredential<PoseidonHasher, PoseidonSignature>,
)>,Status> {
    let data = data.0;

    let mut ledger_authority: RemoteLedgerAuthority<PoseidonHasher, 2> = RemoteLedgerAuthority::new(LEDGER_URL.to_owned());

    let nfts_wrap = data.nft.transfer_split(&data.private_key, data.fraction, &data.target_a, &data.target_b, &mut ledger_authority);

    match nfts_wrap {
    Ok(nfts) => Ok(Json(nfts)),
    Err(_) => Err(Status::InternalServerError),
    }
}


#[post("/burn", data = "<data>", format = "json")]
fn burn(data: Json<BurnParams>) -> Result<Json<NFTCredential<PoseidonHasher, PoseidonSignature>>,Status> {
    let mut data = data.0;

    let mut ledger_authority = RemoteLedgerAuthority::new(LEDGER_URL.to_owned());

    let nft_wrap = data.nft.burn(&data.private_key, &mut ledger_authority);

    match nft_wrap {
    Ok(_) => Ok(Json(data.nft)),
    Err(_) => Err(Status::InternalServerError),
    }

}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .configure(rocket::Config::figment().merge(("port", 9797)))
    .mount("/", routes![
        index,
        new,
        transfer_split,
        burn
        ])
}



#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::thread::sleep;
    use std::time::Duration;

    use crate::NewParams;

    use super::*;
    use num_bigint::BigInt;
    use rocket::local::blocking::{Client, LocalResponse};

   

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let trusted_priv = BigInt::from_str("1").unwrap();
        let trusted_pub = BigInt::from_str(
            "18586133768512220936620570745912940619677854269274689475585506675881198879027",
        )
        .unwrap();

        let owner_a_priv = BigInt::from_str("2").unwrap();
        let owner_a_pub = BigInt::from_str(
            "8645981980787649023086883978738420856660271013038108762834452721572614684349",
        )
        .unwrap(); //poseidon of 2

        let owner_b_priv = BigInt::from_str("3").unwrap();
        let owner_b_pub = BigInt::from_str(
            "6018413527099068561047958932369318610297162528491556075919075208700178480084",
        )
        .unwrap(); //poseidon of 3

        println!("Ledger authority init start");

        let mut trust_list = vec![
            trusted_pub.to_string(),
            "2".to_owned(),
            "3".to_owned(),
            "4".to_owned(),
            "5".to_owned(),
        ];

        for _ in trust_list.len()..((2 as u32).pow(10) as usize) {
            trust_list.push("0".to_owned());
        }
        println!("Ledger authority init finished");

        //
        // minting
        //
        println!("Testing minting");

        //default case
        dbg!("Proof created");

        let params = NewParams{
            attributes: vec![
                "John".to_owned(),
                "Jones".to_owned(),
                "male".to_owned(),
                "843995700".to_owned(),
                "blue".to_owned(),
                "180".to_owned(),
                "115703781".to_owned(),
                "499422598".to_owned(),
            ],
            id:255 as u64,
            expiration:365,
            credential_type: "test".to_owned(),
            delegatable:false,
            registry: "revocRegistry".to_owned(),
            sk_issuer: trusted_priv.clone(),
            owner:owner_a_pub.clone(),
            trust_list_index: 0
        };


        let rock_respionse = client.post(uri!(super::new)).json(&params).dispatch();
        let response: LocalResponse = rock_respionse;
        let nft : Option<NFTCredential<PoseidonHasher, PoseidonSignature>> = response.into_json();
        let nft = nft.expect("msg");
        
        //
        // testing transfer
        // 

        sleep(Duration::from_secs(15));
            
        let params = TransferParams{
            nft,
            private_key: owner_a_priv.clone(),
            fraction:[BigInt::from_str("1").unwrap(), BigInt::from_str("2").unwrap()],
            target_a:owner_a_pub.clone(),
            target_b: owner_b_pub.clone(),
        };
        let rock_respionse = client.post(uri!(super::transfer_split)).json(&params).dispatch();
        let response: LocalResponse = rock_respionse;
        dbg!(&response.status());
        let nft : Option<(NFTCredential<PoseidonHasher, PoseidonSignature>,NFTCredential<PoseidonHasher, PoseidonSignature>)> = response.into_json();
        let  (nft_a, nft_b) = nft.expect("msg");
        
        //
        // testing burn
        // 

        sleep(Duration::from_secs(15));
        
        let params = BurnParams{
            nft: nft_a,
            private_key: owner_a_priv.clone(),
        };
        let rock_respionse = client.post(uri!(super::burn)).json(&params).dispatch();
        let response: LocalResponse = rock_respionse;
        dbg!(&response.status());
    }
}