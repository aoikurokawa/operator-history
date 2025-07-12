use helpers::TestBuilder;
use operator_history_core::config::Config;
use solana_signer::Signer;

mod helpers;

#[tokio::test]
async fn initialize_config() {
    let fixture = TestBuilder::new().await;

    let mut operator_history_program_client = fixture.operator_history_program_client();

    let config = Config::find_program_address(&operator_history_program::id()).0;

    let config_admin = operator_history_program_client
        .do_initialize_config()
        .await
        .unwrap();

    let config = operator_history_program_client
        .get_config(&config)
        .await
        .unwrap();

    assert_eq!(config.admin(), config_admin.pubkey());
    assert_eq!(config.jito_vault_program_id(), jito_vault_program::id());
}
