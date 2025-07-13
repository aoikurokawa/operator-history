use helpers::TestBuilder;
use operator_history_core::{config::Config, operator_history::OperatorHistory};
use solana_signer::Signer;

mod helpers;

#[tokio::test]
async fn initialize_operator_history_account() {
    let fixture = TestBuilder::new().await;

    let mut jito_restaking_program_client = fixture.jito_restaking_program_client();
    let mut operator_history_program_client = fixture.operator_history_program_client();

    jito_restaking_program_client
        .do_initialize_config()
        .await
        .unwrap();
    let operator_root = jito_restaking_program_client
        .do_initialize_operator()
        .await
        .unwrap();

    operator_history_program_client
        .do_initialize_config()
        .await
        .unwrap();
    operator_history_program_client
        .do_initialize_operator_history_account(&operator_root.operator_pubkey)
        .await
        .unwrap();

    let operator_history = OperatorHistory::find_program_address(
        &operator_history_program::id(),
        &operator_root.operator_pubkey,
    )
    .0;
    let operator_history: OperatorHistory = operator_history_program_client
        .get_account(&operator_history)
        .await
        .unwrap();

    // assert_eq!(config.admin(), config_admin.pubkey());
    // assert_eq!(
    //     config.jito_restaking_program_id(),
    //     jito_restaking_program::id()
    // );
}
