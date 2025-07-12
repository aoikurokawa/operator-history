#[cfg(test)]
mod tests {
    use mollusk_svm::{result::Check, Mollusk};
    use solana_account::Account;
    use solana_pubkey::Pubkey;

    #[test]
    fn test_initialize_operator_history() {
        let sender = Pubkey::new_unique();
        let recipient = Pubkey::new_unique();

        let base_lamports = 100_000_000u64;
        let transfer_amount = 42_000u64;

        let instruction =
            solana_system_interface::instruction::transfer(&sender, &recipient, transfer_amount);
        let accounts = [
            (
                sender,
                Account::new(base_lamports, 0, &solana_system_interface::program::id()),
            ),
            (
                recipient,
                Account::new(base_lamports, 0, &solana_system_interface::program::id()),
            ),
        ];
        let checks = vec![
            Check::success(),
            // Check::compute_units(system_processor::DEFAULT_COMPUTE_UNITS),
            Check::account(&sender)
                .lamports(base_lamports - transfer_amount)
                .build(),
            Check::account(&recipient)
                .lamports(base_lamports + transfer_amount)
                .build(),
        ];

        Mollusk::default().process_and_validate_instruction(&instruction, &accounts, &checks);
    }
}
