use anyhow::Result;
use relayer_utils::LOG;
use sdk_utils::run_command;
use serde::{Deserialize, Serialize};
use slog::info;

#[derive(Deserialize, Serialize, Debug)]
pub struct Proof {
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
    pub protocol: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(transparent)]
pub struct PublicOutputs(pub Vec<String>);

pub async fn prove(blueprint_path: &str, proof_id: &str) -> Result<()> {
    info!(LOG, "blueprint_path: {}", blueprint_path);

    // Generate witness
    info!(LOG, "Generating witness");
    run_command(
        "./circuit",
        &[
            &format!("input_{}.json", proof_id),
            &format!("witness_{}.wtns", proof_id),
        ],
        Some(blueprint_path),
    )
    .await?;

    // Generate the proof
    info!(LOG, "Generating proof");
    // run_command(
    //     "prover",
    //     &[
    //         "circuit.zkey",
    //         &format!("witness_{}.wtns", proof_id),
    //         &format!("proof_{}.json", proof_id),
    //         &format!("public_{}.json", proof_id),
    //     ],
    //     Some(blueprint_path), // Assuming this sets the working directory
    // ).await?;

    run_command(
        "snarkjs",
        &[
            "groth16",
            "prove",
            "circuit.zkey",
            &format!("witness_{}.wtns", proof_id),
            &format!("proof_{}.json", proof_id),
            &format!("public_{}.json", proof_id),
        ],
        Some(blueprint_path), // Assuming this sets the working directory
    ).await?;

    Ok(())
}

pub fn read_proof_and_public_data(
    artifacts_dir: &str,
    proof_id: &str,
) -> Result<(Proof, PublicOutputs)> {
    let proof = std::fs::read_to_string(format!("{}/proof_{}.json", artifacts_dir, proof_id))?;
    let public = std::fs::read_to_string(format!("{}/public_{}.json", artifacts_dir, proof_id))?;

    let proof: Proof = serde_json::from_str(&proof)?;
    let public: PublicOutputs = serde_json::from_str(&public)?;

    Ok((proof, public))
}

pub fn cleanup(artifacts_dir: &str, proof_id: &str) -> Result<()> {
    std::fs::remove_file(format!("{}/input_{}.json", artifacts_dir, proof_id))?;
    std::fs::remove_file(format!("{}/witness_{}.wtns", artifacts_dir, proof_id))?;
    std::fs::remove_file(format!("{}/proof_{}.json", artifacts_dir, proof_id))?;
    std::fs::remove_file(format!("{}/public_{}.json", artifacts_dir, proof_id))?;

    Ok(())
}
