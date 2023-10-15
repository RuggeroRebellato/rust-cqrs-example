#![allow(unused_imports)]
// Our demo will use MemStore as database, but you can use any
// database you want as long as it implements the Store trait.
use async_trait::async_trait;
use cqrs_es::CqrsFramework;
use cqrs_es::{mem_store::MemStore, EventEnvelope, Query};

use crate::domain::aggregate::BankAccount;
use crate::domain::commands::BankAccountCommand;
use crate::services::BankAccountServices;

struct SimpleLogginQuery {}

#[async_trait]
impl Query<BankAccount> for SimpleLogginQuery {
    async fn dispatch(&self, aggregate_id: &str, events: &[EventEnvelope<BankAccount>]) {
        for event in events {
            println!("{}-{}\n{:#?}", aggregate_id, event.sequence, &event.payload);
        }
    }
}

#[tokio::test]
async fn test_event_store() {
    let event_store = MemStore::<BankAccount>::default();
    let query = SimpleLogginQuery {};
    let cqrs = CqrsFramework::new(event_store, vec![Box::new(query)], BankAccountServices);

    let aggregate_id = "aggregate-instance-A";

    cqrs.execute(
        aggregate_id,
        BankAccountCommand::DepositMoney { amount: 1000.00 },
    )
    .await
    .unwrap();

    cqrs.execute(
        aggregate_id,
        BankAccountCommand::WithdrawMoney { amount: 500.00 },
    )
    .await
    .unwrap();
}
