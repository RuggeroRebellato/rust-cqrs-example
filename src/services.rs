/*
    A Service is a stateless operation that is not a part of the domain. It is a part of the application layer. Services are always in the imperative tense, and should be named in a way that makes it clear what will happen.
    One example of a service is a payment gateway. This is a third party service that we will need to interact with in order to complete a command. In this case we will need to send a request to the payment gateway and then wait for a response. This is a good example of a service because it is not a part of our domain, but it is a part of our application.
*/

pub struct BankAccountServices;

impl BankAccountServices {
    async fn _atm_withdrawal(&self, _atm_id: &str, _amount: f64) -> Result<(), _AtomError> {
        Ok(())
    }

    async fn _validate_check(&self, _account: &str, _check: &str) -> Result<(), _CheckingError> {
        Ok(())
    }
}

pub struct _AtomError;
pub struct _CheckingError;
