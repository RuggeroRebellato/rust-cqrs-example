/*
    we qualify events with 'domain' to differentiate them from other events that might exist within our application. These are domain events because they make assertions about changes in the aggregate state.

    In the cqrs-es framework the domain events are expected to be an enum with payloads similar to the commands, this will give us a single root event for each aggregate.

    Events are always in the past tense, and should be named in a way that makes it clear what happened. The event payloads should contain all of the information needed to rebuild the aggregate state.

    e.g: CustomerDepositedMoney { amount: f64, balance: f64 }

    The enum as well as the payloads should derive several traits.

    Debug - used for error handling and testing.
    Clone - the event may be passed to a number of downstream queries in an asynchronous manner and will need to be cloned.
    Serialize, Deserialize - serialization is essential for both storage and publishing to distributed queries.
    PartialEq - we will be adding a lot of tests to verify that our business logic is correct.
*/

use cqrs_es::DomainEvent;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BankAccountEvent {
    AccountOpened {
        account_id: String,
    },
    CustomerDepositedMoney {
        amount: f64,
        balance: f64,
    },
    CustomerWithdrewCash {
        amount: f64,
        balance: f64,
    },
    CustomerWroteCheck {
        check_number: String,
        amount: f64,
        balance: f64,
    },
}

impl DomainEvent for BankAccountEvent {
    fn event_type(&self) -> String {
        let event_type: &str = match self {
            BankAccountEvent::AccountOpened { .. } => "AccountOpened",
            BankAccountEvent::CustomerDepositedMoney { .. } => "CustomerDepositedMoney",
            BankAccountEvent::CustomerWithdrewCash { .. } => "CustomerWithdrewCash",
            BankAccountEvent::CustomerWroteCheck { .. } => "CustomerWroteCheck",
        };

        event_type.to_string()
    }

    fn event_version(&self) -> String {
        "1.0".to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct BankAccountError(String);

impl Display for BankAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BankAccountError {}

impl From<&str> for BankAccountError {
    fn from(message: &str) -> Self {
        Self(message.to_string())
    }
}
