pub struct BankAccountServices;

impl BankAccountServices {
    async fn atm_withdrawal(&self, atm_id: &str, amount: f64) -> Result<(), AtomError> {
        Ok(())
    }

    async fn validate_check(&self, account: &str, check: &str) -> Result<(), CheckingError> {
        Ok(())
    }
}

pub struct AtomError;
pub struct CheckingError;
