use operator_history_core::{config::Config, operator_history::OperatorHistory};
use operator_history_sdk::sdk::{
    initialize_config, initialize_operator_history_account, realloc_operator_history_account,
};
use solana_commitment_config::CommitmentLevel;
use solana_keypair::Keypair;
use solana_native_token::sol_str_to_lamports;
use solana_program_test::BanksClient;
use solana_pubkey::Pubkey;
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;

use super::TestError;

pub struct OperatorHistoryProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl OperatorHistoryProgramClient {
    /// Construct new `OperatorHistoryProgramClient`
    pub const fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn airdrop(&mut self, to: &Pubkey, sol: f64) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(
                        &self.payer.pubkey(),
                        to,
                        sol_str_to_lamports(&sol.to_string()).unwrap(),
                    )],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    /// Process the transaction
    pub async fn process_transaction(&mut self, tx: &Transaction) -> Result<(), TestError> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    /// Get operator history program account
    #[allow(dead_code)]
    pub async fn get_account<T>(&mut self, account: &Pubkey) -> Result<T, TestError>
    where
        T: jito_bytemuck::AccountDeserialize,
    {
        let raw_account = self.banks_client.get_account(*account).await?.unwrap();
        let account = *T::try_from_slice_unchecked(raw_account.data.as_slice())?;

        Ok(account)
    }

    /// Execute initialize_config instruction
    pub async fn do_initialize_config(&mut self) -> Result<Keypair, TestError> {
        let config_pubkey = Config::find_program_address(&operator_history_program::id()).0;
        let config_admin = Keypair::new();

        self.airdrop(&config_admin.pubkey(), 1.0).await?;
        self.initialize_config(&config_pubkey, &config_admin)
            .await?;

        Ok(config_admin)
    }

    /// Initialize config
    pub async fn initialize_config(
        &mut self,
        config: &Pubkey,
        config_admin: &Keypair,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_config(
                &operator_history_program::id(),
                config,
                &config_admin.pubkey(),
                &jito_restaking_program::id(),
            )],
            Some(&config_admin.pubkey()),
            &[config_admin],
            blockhash,
        ))
        .await
    }

    #[allow(dead_code)]
    pub async fn do_initialize_operator_history_account(
        &mut self,
        operator: &Pubkey,
    ) -> Result<(), TestError> {
        let config = Config::find_program_address(&operator_history_program::id()).0;
        let operator_history_pubkey =
            OperatorHistory::find_program_address(&operator_history_program::id(), operator).0;
        let payer = Keypair::new();

        self.airdrop(&payer.pubkey(), 100.0).await?;
        self.initialize_operator_history_account(
            &config,
            &operator_history_pubkey,
            &operator,
            &payer,
        )
        .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn initialize_operator_history_account(
        &mut self,
        config: &Pubkey,
        operator_history: &Pubkey,
        operator: &Pubkey,
        payer: &Keypair,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_operator_history_account(
                &operator_history_program::id(),
                config,
                &operator_history,
                &operator,
                &payer.pubkey(),
            )],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ))
        .await
    }

    #[allow(dead_code)]
    pub async fn do_realloc_operator_history_account(
        &mut self,
        operator: &Pubkey,
    ) -> Result<(), TestError> {
        let config = Config::find_program_address(&operator_history_program::id()).0;
        let operator_history_pubkey =
            OperatorHistory::find_program_address(&operator_history_program::id(), operator).0;
        let payer = Keypair::new();

        self.airdrop(&payer.pubkey(), 100.0).await?;
        self.realloc_operator_history_account(&config, &operator_history_pubkey, &operator, &payer)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn realloc_operator_history_account(
        &mut self,
        config: &Pubkey,
        operator_history: &Pubkey,
        operator: &Pubkey,
        payer: &Keypair,
    ) -> Result<(), TestError> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[realloc_operator_history_account(
                &operator_history_program::id(),
                config,
                &operator_history,
                &operator,
                &payer.pubkey(),
            )],
            Some(&payer.pubkey()),
            &[payer],
            blockhash,
        ))
        .await
    }
}
