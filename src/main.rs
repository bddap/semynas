mod mirrors;

use dock_testnet_runtime::{Call, Runtime};
use parity_scale_codec::Encode;
use sp_runtime::traits::Hash;

type Hasher = <Runtime as frame_system::Trait>::Hashing;

fn main() {
    example_runtime_upgrade();
}

fn example_runtime_upgrade() {
    let wasm = include_bytes!("runtime.wasm");
    let call = Call::System(frame_system::Call::set_code(wasm.to_vec()));
    let hash = Hasher::hash_of(&call);
    dbg!(&call);
    dbg!(&hash);
    dbg!(&call.encode());
}

// Things to try:

// propose a runtime upgrade
// attempt to close and execute, should fail

// propose a runtime upgrade
// vote yes from one other account
// attempt to close and execute, should succeed
