use diesel::{prelude::*, sql_query};
use eyre::Result;
use diesel_async::RunQueryDsl;
use crate::{
  connection::PostgresConnection,
  models::{
    cnt::{CNT, CNTWithMetadata, PartialListing, CNTWithEvent},
    ticket_type::TicketType,
    event::Event, seat_range::SeatRange, nft_detail::TicketTypeNftDetail, nft::TicketTypeNft,
  },
  schema::{
    cnts::dsl::{self as cnts_dsl},
  },
};

impl PostgresConnection {
  pub async fn upsert_user_cnt(&mut self, user_cnt: CNT) -> Result<()> {
    diesel::insert_into(cnts_dsl::cnts)
    .values(&user_cnt)
    .on_conflict((cnts_dsl::event_id, cnts_dsl::seat_index))
    .do_update()
    .set(&user_cnt)
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }
  pub async fn set_cnt_sui_address(&mut self, event_id: String, seat_index: i32, cnt_sui_address: String) -> Result<()> {
    diesel::update(cnts_dsl::cnts)
    .filter(
      cnts_dsl::event_id.eq(event_id).and(cnts_dsl::seat_index.eq(seat_index))
    )
    .set((
      cnts_dsl::cnt_sui_address.eq(cnt_sui_address),
      cnts_dsl::draft.eq(false)
    ))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn read_cnts_for_event(&mut self, evt_id: String, skip: i64, limit: i64) -> Result<Vec<CNTWithMetadata>> {
    let query = sql_query(format!(
      "
      SELECT DISTINCT
        cnts.*,
        ticket_types.*,
        seat_ranges.*,
        ticket_type_nft_details.*,
        nft_details.*,
        ticket_type_nfts.*,
        listings.listing_sui_address
      FROM (
        SELECT * FROM cnts
        WHERE cnts.event_id = '{}' AND cnts.draft = FALSE
        LIMIT {}
        OFFSET {}
      ) cnts
      INNER JOIN ticket_types USING(event_id, ticket_type_index)
      INNER JOIN seat_ranges USING(event_id, ticket_type_index)
      INNER JOIN ticket_type_nft_details USING(event_id, ticket_type_index)
      INNER JOIN nft_details ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
      LEFT JOIN ticket_type_nfts USING(ref_name, cnt_sui_address)
      LEFT JOIN listings ON (
        listings.event_id = cnts.event_id
        AND listings.cnt_sui_address = cnts.cnt_sui_address
        AND listings.is_open = TRUE AND listings.draft = FALSE
      )
      ORDER BY cnts.created_at
      ", evt_id, limit, skip * limit
    ));

    let records = query
    .load::<(CNT, TicketType, SeatRange, TicketTypeNftDetail, Option<TicketTypeNft>, Option<PartialListing>)>(self.borrow_mut())
    .await?;

    Ok(CNTWithMetadata::from_tuple(records))
  }

  pub async fn read_user_cnts(
    &mut self,
    uid: String,
    event_id: Option<String>,
    skip: i64,
    limit: i64,
  ) -> Result<Vec<CNTWithEvent>> {
    let event_id_filter = event_id.map_or("".to_string(), |event_id| {
      format!("AND cnts.event_id = '{}'", event_id)
    });

    let query = sql_query(format!(
      "
      SELECT DISTINCT
        cnts.*,
        events.*,
        ticket_types.*,
        seat_ranges.*,
        ticket_type_nft_details.*,
        nft_details.*,
        ticket_type_nfts.*,
        listings.listing_sui_address
      FROM (
        SELECT * FROM cnts
        WHERE cnts.account_id = '{}' AND cnts.draft = FALSE {}
        LIMIT {}
        OFFSET {}
      ) cnts
      INNER JOIN events USING(event_id)
      INNER JOIN ticket_types USING(event_id, ticket_type_index)
      INNER JOIN seat_ranges USING(event_id, ticket_type_index)
      INNER JOIN ticket_type_nft_details USING(event_id, ticket_type_index)
      INNER JOIN nft_details ON nft_details.arweave_tx_id = ticket_type_nft_details.nft_details_id
      LEFT JOIN ticket_type_nfts USING(ref_name, cnt_sui_address)
      LEFT JOIN listings ON (
        listings.event_id = cnts.event_id
        AND listings.cnt_sui_address = cnts.cnt_sui_address
        AND listings.is_open = TRUE AND listings.draft = FALSE
      )
      ORDER BY cnts.created_at
      ",
      uid,
      event_id_filter,
      limit,
      skip * limit,
    ));

    let records = query
    .load::<(CNT, Event, TicketType, SeatRange, TicketTypeNftDetail, Option<TicketTypeNft>, Option<PartialListing>)>(self.borrow_mut())
    .await?;

    Ok(CNTWithEvent::from_tuple(records))
  }

  pub async fn update_attended(&mut self, cnt_sui_address: String) -> Result<()> {
    diesel::update(cnts_dsl::cnts)
    .filter(cnts_dsl::cnt_sui_address.eq(cnt_sui_address))
    .set(cnts_dsl::attended.eq(true))
    .execute(self.borrow_mut())
    .await?;

    Ok(())
  }

  pub async fn has_attended(&mut self, cnt_sui_address: String) -> Result<bool> {
    Ok(
      cnts_dsl::cnts
      .filter(cnts_dsl::cnt_sui_address.eq(cnt_sui_address))
      .select(cnts_dsl::attended)
      .first(self.borrow_mut())
      .await?
    )
  }

  pub async fn read_cnt(&mut self, cnt_sui_address: String) -> Result<Vec<CNT>> {
    Ok(
      cnts_dsl::cnts
      .filter(cnts_dsl::cnt_sui_address.eq(cnt_sui_address))
      .load::<CNT>(self.borrow_mut())
      .await?
    )
  }
}
