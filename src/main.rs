use dock_testnet_runtime::{self as dr, AccountId, Call, Runtime, Signature, SignedExtra};
use futures::{
    compat::{Compat, Future01CompatExt},
    future::FutureExt,
};
use jsonrpc_client_transports::transports::ws;
use jsonrpc_client_transports::RpcError;
use parity_scale_codec::Encode;
use sc_rpc::author::gen_client::Client;
use sp_core::{blake2_256, Bytes};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::generic::Era;
use sp_runtime::traits::SignedExtension;
// use parity_scale_codec::Encode;
// use sc_rpc::author::AuthorApi;
// use sp_runtime::traits::Hash;

// type Hasher = <Runtime as frame_system::Trait>::Hashing;
type Hash = <Runtime as frame_system::Trait>::Hash;
type BlockHash = Hash; // assuming the two are the same

fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(Compat::new(amain().boxed()))
        .unwrap();
}

async fn amain() -> Result<(), RpcError> {
    let conn = ws::connect(&"ws://127.0.0.1:9944".parse().unwrap())
        .compat()
        .await?;
    let client = Client::<Hash, BlockHash>::new(conn);
    remark(&client).await;
    Ok(())
}

async fn remark(cl: &Client<Hash, BlockHash>) {
    let call = Call::System(frame_system::Call::remark(vec![]));
    let extra: SignedExtra = (
        frame_system::CheckSpecVersion::new(),
        frame_system::CheckTxVersion::new(),
        frame_system::CheckGenesis::new(),
        frame_system::CheckEra::from(Era::Immortal),
        frame_system::CheckNonce::from(0),
        frame_system::CheckWeight::new(),
        pallet_transaction_payment::ChargeTransactionPayment::from(0u64),
        token_migration::OnlyMigrator::new(),
    );
    let additional = (
        0u32,
        0u32,
        [0u8; 32].into(),
        [0u8; 32].into(),
        (),
        (),
        (),
        (),
    );
    let extrinsic = sign_as_alice(call, extra, additional);
    cl.submit_extrinsic(Bytes(extrinsic.encode()))
        .compat()
        .await
        .unwrap();
}

fn sign_as_alice(
    call: Call,
    extra: SignedExtra,
    additional_signed: <SignedExtra as SignedExtension>::AdditionalSigned,
) -> dr::UncheckedExtrinsic {
    let encoded = (call.clone(), extra.clone(), additional_signed).encode();
    let hash = blake2_256(&encoded);
    let payload: &[u8] = if encoded.len() > 256 { &hash } else { &encoded };
    let kp = dev_secret_from_seed::<sr25519::Public>("Alice");
    let sig: Signature = kp.sign(payload).into();
    let account: AccountId = AccountId::from(kp.public());
    dr::UncheckedExtrinsic::new_signed(call, account, sig, extra)
}

fn dev_secret_from_seed<TPublic: Public>(seed: &str) -> TPublic::Pair {
    TPublic::Pair::from_string(&format!("//{}", seed), None).unwrap()
}

// let call = Call::System(frame_system::Call::set_storage(vec![]));
// dbg!(&call.encode());
