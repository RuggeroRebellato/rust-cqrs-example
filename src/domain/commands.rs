/**
A command is a request to change the state of the system. It is not a request to read the state of the system. Commands are always in the imperative tense, and should be named in a way that makes it clear what will happen.

In order to make changes to our system we will need commands. These are the simplest components of any CQRS system and consist of little more than packaged data.

When designing commands an easy mental model to use is that of an HTTP API. Each virtual endpoint would receive just the data that is needed to operate that function.
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BankAccountCommand {
    OpenAccount { account_id: String },
    DepositMoney { amount: f64 },
    WithdrawMoney { amount: f64 },
    WriteCheck { check_number: String, amount: f64 },
}

// The most common way to receive commands from a user is
// via an HTTP body that can be directly deserialized.
