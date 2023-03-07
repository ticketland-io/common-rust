use diesel::prelude::*;
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{stripe_account::StripeAccount, stripe_customer::StripeCustomer},
  schema::{
    stripe_accounts::dsl::{
      self as stripe_accounts_dsl,
      stripe_accounts,
    },
    accounts::dsl::{
      self as accounts_dsl,
      accounts,
    },
    events::dsl::{
      self as events_dsl,
      events,
    },
    stripe_customers::dsl::{
      self as stripe_customers_dsl,
      stripe_customers,
    }
  },
};

impl PostgresConnection {
  pub async fn upsert_stripe_account(&mut self, account: StripeAccount) -> Result<()> {
    diesel::insert_into(stripe_accounts)
    .values(&account)
    .on_conflict(stripe_accounts_dsl::stripe_uid)
    .do_update()
    .set(&account)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn update_stripe_account_status(&mut self, stripe_id: String) -> Result<()> {
    diesel::update(stripe_accounts)
    .filter(stripe_accounts_dsl::stripe_uid.eq(stripe_id))
    .set(stripe_accounts_dsl::status.eq(1))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_stripe_account(&mut self, user_id: String) -> Result<StripeAccount> {
    Ok(
      stripe_accounts
      .select(stripe_accounts::all_columns())
      .filter(stripe_accounts_dsl::account_id.eq(user_id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(stripe_accounts_dsl::account_id)))
      .first::<StripeAccount>(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_event_organizer_stripe_account(&mut self, event_id: String) -> Result<StripeAccount> {
    Ok(
      events
      .filter(events_dsl::event_id.eq(event_id))
      .inner_join(accounts.on(accounts_dsl::uid.eq(events_dsl::account_id)))
      .inner_join(stripe_accounts.on(stripe_accounts_dsl::account_id.eq(events_dsl::account_id)))
      .select(stripe_accounts::all_columns())
      .first::<StripeAccount>(self.borrow_mut())
      .await?
    )
  }

  pub async fn upsert_stripe_customer(&mut self, customer: StripeCustomer) -> Result<()> {
    diesel::insert_into(stripe_customers)
    .values(&customer)
    .on_conflict(stripe_customers_dsl::customer_uid)
    .do_update()
    .set(&customer)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_stripe_customer(&mut self, account_id: String) -> Result<StripeCustomer> {
    Ok(
      stripe_customers
      .filter(stripe_customers_dsl::account_id.eq(account_id))
      .first::<StripeCustomer>(self.borrow_mut())
      .await?
    )
  }
}
