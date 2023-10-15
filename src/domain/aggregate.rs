/*

An aggregate is a state machine that is driven by commands and produces events. The aggregate is responsible for validating the commands and producing the events. The aggregate is also responsible for applying the events to its own state and is the only component that is allowed to mutate its own state.

Command and event are in place we can now start adding our business logic. In Domain Driven Design all of this logic belongs within the aggregate which for our example we will call name BankAccount. And for our simple set of business rules, we will use two fields. The first is a boolean to indicate if the account is open or closed. The second is a balance field to track the current balance of the account.

*/

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
        command: Self::Command,
        _services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            BankAccountCommand::WithdrawMoney { amount } => {
                let balance = self.balance - amount;
                if balance < 0_f64 {
                    return Err("Insufficient funds".into());
                }
                Ok(vec![BankAccountEvent::CustomerWithdrewCash {
                    amount,
                    balance,
                }])
            }
            BankAccountCommand::DepositMoney { amount } => {
                let balance = self.balance + amount;
                Ok(vec![BankAccountEvent::CustomerDepositedMoney {
                    amount,
                    balance,
                }])
            }
            _ => Ok(vec![]),
        }
    }

    // The aggregate's apply() method is responsible for applying events to the
    // aggregate.
    // This method is called by the event store when loading events from the event store.

    fn apply(&mut self, event: Self::Event) {
        match event {
            BankAccountEvent::AccountOpened { .. } => {
                self.opened = true;
            }
            BankAccountEvent::CustomerDepositedMoney { balance, .. }
            | BankAccountEvent::CustomerWithdrewCash { balance, .. }
            | BankAccountEvent::CustomerWroteCheck { balance, .. } => {
                self.balance = balance;
            }
        }
    }
}

/*
tests follow the pattern:
    Given some past events
    When a command is applied
    Then some result is expected
*/
#[cfg(test)]
mod aggregate_tests {

    use super::*;
    use cqrs_es::test::TestFramework;

    // The TestFramework is a helper for testing aggregates and is provided by
    // the cqrs-es crate. It provides a simple way to test aggregates by using
    // the aggregate's handle() method to handle commands and return events.
    // The TestFramework also provides a way to apply events to an aggregate and
    // assert that the aggregate's state is correct.
    // The TestFramework is generic over the aggregate type, so we need to
    // define a type alias for the TestFramework with the BankAccount type.

    type AccountTestFramework = TestFramework<BankAccount>;

    #[test]
    fn test_deposit_money() {
        let expected = BankAccountEvent::CustomerDepositedMoney {
            amount: 100.0,
            balance: 100.0,
        };
        let command = BankAccountCommand::DepositMoney { amount: 100.0 };
        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(command)
            .then_expect_events(vec![expected])
    }

    #[test]
    fn test_deposit_money_with_balance() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerDepositedMoney {
            amount: 100.0,
            balance: 300.0,
        };
        let command = BankAccountCommand::DepositMoney { amount: 100.0 };
        let services = BankAccountServices;

        AccountTestFramework::with(services)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected])
    }

    #[test]
    fn test_withdraw_money() {
        let previous = BankAccountEvent::CustomerDepositedMoney {
            amount: 200.0,
            balance: 200.0,
        };
        let expected = BankAccountEvent::CustomerWithdrewCash {
            amount: 100.0,
            balance: 100.0,
        };
        let command = BankAccountCommand::WithdrawMoney { amount: 100.0 };
        let services = BankAccountServices;

        AccountTestFramework::with(services)
            .given(vec![previous])
            .when(command)
            .then_expect_events(vec![expected])
    }

    #[test]
    fn test_withdraw_money_funds_not_available() {
        AccountTestFramework::with(BankAccountServices)
            .given_no_previous_events()
            .when(BankAccountCommand::WithdrawMoney { amount: 200.0 })
            .then_expect_error("Insufficient funds".into())
    }
}
