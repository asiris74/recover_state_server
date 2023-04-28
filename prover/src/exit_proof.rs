//! Generate exit proof for exodus mode given account and token

use crate::proving_cache::ProvingCache;
use anyhow::format_err;
use num::BigUint;
use recover_state_config::RecoverStateConfig;
use std::fs::File;
use std::time::Instant;
use tracing::info;
use zklink_circuit::witness::create_exit_circuit_with_public_input;
use zklink_crypto::bellman::plonk::better_cs::{keys::VerificationKey, verifier::verify};
use zklink_crypto::bellman::plonk::{
    commitments::transcript::keccak_transcript::RollingKeccakTranscript, prove_by_steps,
};
use zklink_crypto::circuit::CircuitAccountTree;
use zklink_crypto::franklin_crypto::bellman::Circuit;
use zklink_crypto::proof::EncodedSingleProof;
use zklink_crypto::proof::SingleProof;
use zklink_crypto::{Engine, Fr};
use zklink_types::{AccountId, ChainId, SubAccountId, TokenId};

#[allow(clippy::too_many_arguments)]
pub fn create_exit_proof(
    config: &RecoverStateConfig,
    circuit_account_tree: &CircuitAccountTree,
    cache: &ProvingCache,
    account_id: AccountId,
    sub_account_id: SubAccountId,
    l2_source_token: TokenId,
    l1_target_token: TokenId,
    chain_id: ChainId,
    max_chain_num: usize,
) -> Result<(EncodedSingleProof, BigUint), anyhow::Error> {
    let timer = Instant::now();
    let (exit_circuit, withdraw_amount) = create_exit_circuit_with_public_input(
        circuit_account_tree,
        account_id,
        sub_account_id,
        l2_source_token,
        l1_target_token,
        chain_id,
        max_chain_num,
    );
    info!("Exit witness generated: {} s", timer.elapsed().as_secs());

    let proof = gen_verified_proof_for_exit_circuit(config, exit_circuit, cache)
        .map_err(|e| format_err!("Failed to generate proof: {}", e))?;

    info!("Exit proof created: {} s", timer.elapsed().as_secs());
    Ok((proof.serialize_single_proof(), withdraw_amount))
}

/// Generates proof for exit given circuit using step-by-step algorithm.
pub fn gen_verified_proof_for_exit_circuit<C: Circuit<Engine> + Clone>(
    config: &RecoverStateConfig,
    circuit: C,
    cache: &ProvingCache,
) -> Result<SingleProof, anyhow::Error> {
    let vk = VerificationKey::read(File::open(crate::utils::get_exodus_verification_key_path(
        &config.runtime.key_dir,
    ))?)?;

    info!("Proof for circuit started");
    let proof = prove_by_steps::<_, _, RollingKeccakTranscript<Fr>>(
        circuit,
        &cache.hints,
        &cache.setup,
        None,
        &cache.key_monomial_form,
        None,
    )?;

    let valid = verify::<_, _, RollingKeccakTranscript<Fr>>(&proof, &vk, None)?;
    anyhow::ensure!(valid, "proof for exit is invalid");

    info!("Proof for circuit successful");
    Ok(proof.into())
}
