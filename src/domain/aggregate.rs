use async_trait::async_trait;
use cqrs_es::Aggregate;
use serde::{Deserialize, Serialize};

use crate::domain::commands::BankAccountCommand;
use crate::domain::events::{BankAccountError, BankAccountEvent};
use crate::services::BankAccountServices;

#[derive(Default, Serialize, Deserialize)]
pub struct BankAccount {
    opened: bool,
    balance: f64,
}

#[async_trait]
impl Aggregate for BankAccount {
    type Command = BankAccountCommand;
    type Event = BankAccountEvent;
    type Error = BankAccountError;
    type Services = BankAccountServices;

    // aggregate_type is a unique identifier for the aggregate type, and is used
    // to identify the aggregate type in the event store.
    fn aggregate_type() -> String {
        // This is the only place where the aggregate type is defined.
        //? Why is this a String and not a &str?
        //? Check if this `into()` is a conversion from &str to String. and if it works
        "bank_account".into()
    }

    // The aggregate's handle() method is responsible for handling commands and
    // returning events.
    // all aggregate logic is contained in this method. Use helper functions elsewhere to keep the code clean and readable.
    async fn handle(
        &self,
        _command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        todo!("Implement the aggregate logic here")
    }

    // The aggregate's apply() method is responsible for applying events to the
    // aggregate.
    // This method is called by the event store when loading events from the event store.

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => {
                self.opened = true;
            }
            BankAccountEvent::CustomerDepositedMoney { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWithdrewCash { amount: _, balance } => {
                self.balance = balance;
            }
            BankAccountEvent::CustomerWroteCheck {
                check_number: _,
                amount: _,
                balance,
            } => {
                self.balance = balance;
            }
        }
    }
}
