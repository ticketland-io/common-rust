use diesel::sql_query;
use eyre::Result;
use diesel::result::Error;
use diesel_async::{AsyncConnection, RunQueryDsl};
use crate::{
  connection::PostgresConnection,
  models::nft::NewTicketTypeNft
};

impl PostgresConnection {
  pub async fn upsert_claimed_nfts(
    &mut self,
    ticket_type_nfts: Vec<NewTicketTypeNft>,
  ) -> Result<()> {
    self.borrow_mut()
    .transaction::<_, Error, _>(|conn| Box::pin(async move {
      // We cannot do multiple UPDATEs in one query, so these need to be separate
      for ticket_type_nft in ticket_type_nfts.iter() {
        let update_query = format!(
          "
          INSERT INTO ticket_type_nfts (
            ticket_type_nft_sui_address,
            cnt_sui_address,
            account_id,
            ref_name,
            event_id,
            ticket_type_index
          )
          VALUES (
            '{}',
            '{}',
            '{}',
            '{}',
            '{}',
            {}
          )
          ",
          ticket_type_nft.ticket_type_nft_sui_address,
          ticket_type_nft.cnt_sui_address,
          ticket_type_nft.account_id,
          ticket_type_nft.ref_name,
          ticket_type_nft.event_id,
          ticket_type_nft.ticket_type_index,
        );

        sql_query(update_query).execute(conn).await?;
      }

      Ok(())
    }))
    .await?;


    Ok(())
  }
}
