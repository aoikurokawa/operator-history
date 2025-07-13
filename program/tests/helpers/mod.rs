use operator_history_program_client::OperatorHistoryProgramClient;
use restaking_program_client::RestakingProgramClient;
use solana_commitment_config::CommitmentLevel;
use solana_native_token::sol_str_to_lamports;
use solana_program_error::ProgramError;
use solana_program_test::{processor, BanksClientError, ProgramTest, ProgramTestContext};
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use thiserror::Error;

mod operator_history_program_client;
mod restaking_program_client;

#[derive(Debug, Error)]
pub enum TestError {
    #[error(transparent)]
    BanksClientError(#[from] BanksClientError),

    #[error(transparent)]
    ProgramError(#[from] ProgramError),
}

pub struct TestBuilder {
    context: ProgramTestContext,
}

impl TestBuilder {
    pub async fn new() -> Self {
        let mut program_test = ProgramTest::new(
            "operator_history_program",
            operator_history_program::id(),
            processor!(operator_history_program::process_instruction),
        );
        program_test.prefer_bpf(true);
        program_test.add_program("jito_restaking_program", jito_restaking_program::id(), None);

        let context = program_test.start_with_context().await;
        Self { context }
    }

    pub fn jito_restaking_program_client(&self) -> RestakingProgramClient {
        RestakingProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    pub fn operator_history_program_client(&self) -> OperatorHistoryProgramClient {
        OperatorHistoryProgramClient::new(
            self.context.banks_client.clone(),
            self.context.payer.insecure_clone(),
        )
    }

    /// Transfer SOL
    #[allow(dead_code)]
    pub async fn transfer(&mut self, to: &Pubkey, sol: f64) -> Result<(), BanksClientError> {
        let blockhash = self.context.banks_client.get_latest_blockhash().await?;
        self.context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(
                        &self.context.payer.pubkey(),
                        to,
                        sol_str_to_lamports(&sol.to_string()).unwrap(),
                    )],
                    Some(&self.context.payer.pubkey()),
                    &[&self.context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
    }
}
