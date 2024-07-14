use crate::v2::{
    engine::state::instrument::OrderManager,
    execution::error::ExecutionError,
    order::{Cancelled, ClientOrderId, InFlight, Open, Order, OrderState},
    Snapshot,
};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use tracing::{debug, error, warn};
use vecmap::VecMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Constructor)]
pub struct Orders<InstrumentKey> {
    pub in_flights: VecMap<ClientOrderId, Order<InstrumentKey, InFlight>>,
    pub opens: VecMap<ClientOrderId, Order<InstrumentKey, Open>>,
}

impl<InstrumentKey> OrderManager<InstrumentKey> for Orders<InstrumentKey>
where
    InstrumentKey: Debug + Display + Clone + PartialEq,
{
    fn record_in_flights(
        &mut self,
        requests: impl IntoIterator<Item = Order<InstrumentKey, InFlight>>,
    ) {
        for request in requests {
            if let Some(duplicate_cid_order) = self.in_flights.insert(request.cid, request) {
                error!(
                    cid = %duplicate_cid_order.cid,
                    event = ?duplicate_cid_order,
                    "OrderManager upserted Order<InFlight> with duplicate ClientOrderId"
                );
            }
        }
    }

    fn update_from_open(&mut self, response: &Order<InstrumentKey, Result<Open, ExecutionError>>) {
        match (self.in_flights.remove(&response.cid), &response.state) {
            (Some(in_flight), Ok(open)) => {
                debug!(
                    instrument = %response.instrument,
                    cid = %response.cid,
                    ?in_flight,
                    open = ?response,
                    "OrderManager removed Order<InFlight> after receiving Order<Open>"
                );

                self.opens
                    .entry(response.cid)
                    .and_modify(|pre_existing| {
                        // Assume pre-existing Order<Open> came from a prior OrderSnapshot
                        // notification, so don't modify with response data
                        error!(
                            instrument = %response.instrument,
                            cid = %response.cid,
                            ?pre_existing,
                            open = ?response,
                            "OrderManager received Order<Open> Ok response for pre-existing Order<Open>"
                        );
                    })
                    .or_insert_with(|| {
                        debug!(
                            instrument = %response.instrument,
                            cid = %response.cid,
                            ?in_flight,
                            open = ?response,
                            "OrderManager added Order<Open> after receiving Order<Open> Ok response"
                        );
                        Order::new(
                            response.instrument.clone(),
                            response.cid,
                            response.side,
                            open.clone(),
                        )
                    });
            }
            (None, Ok(open)) => {
                error!(
                    instrument = %response.instrument,
                    cid = %response.cid,
                    open = ?response,
                    "OrderManager received Order<Open> Ok response for non-InFlight order - why was this not InFlight?"
                );

                self.opens
                    .entry(response.cid)
                    .and_modify(|pre_existing| {
                        // Assume pre-existing Order<Open> came from a prior OrderSnapshot
                        // notification, so don't modify with response data
                        error!(
                            instrument = %response.instrument,
                            cid = %response.cid,
                            ?pre_existing,
                            open = ?response,
                            "OrderManager received Order<Open> Ok response for pre-existing Order<Open>"
                        );
                    })
                    .or_insert_with(|| {
                        debug!(
                            instrument = %response.instrument,
                            cid = %response.cid,
                            open = ?response,
                            "OrderManager added Order<Open> after receiving Order<Open> Ok response"
                        );
                        Order::new(
                            response.instrument.clone(),
                            response.cid,
                            response.side,
                            open.clone(),
                        )
                    });
            }
            (Some(in_flight), Err(error)) => {
                // If InFlight, remove, then log ExecutionError
                // Todo:
                panic!()
            }
            (None, Err(error)) => {
                // if no InFlight, log, then log ExecutionError
                panic!()
            }
        }
    }

    fn update_from_cancel(
        &mut self,
        response: &Order<InstrumentKey, Result<Cancelled, ExecutionError>>,
    ) {
        match &response.state {
            Ok(cancelled) => {
                todo!()
            }
            Err(error) => {
                // Remove from InFlight & log error
                todo!()
            }
        }
    }

    fn update_from_order_snapshot(
        &mut self,
        snapshot: &Snapshot<Order<InstrumentKey, OrderState>>,
    ) {
        let Snapshot(order) = snapshot;

        match &order.state {
            // Remove InFlight order (if present), and upsert the Open Order
            OrderState::Open(open) => {
                if let Some(in_flight) = self.in_flights.remove(&order.cid) {
                    debug!(
                        instrument = %order.instrument,
                        cid = %order.cid,
                        ?in_flight,
                        open = ?order,
                        "OrderManager removed Order<InFlight> after receiving Snapshot<Order<Open>>"
                    );
                }

                if let Some(replaced) = self.opens.insert(
                    order.cid,
                    Order::new(
                        order.instrument.clone(),
                        order.cid,
                        order.side,
                        open.clone(),
                    ),
                ) {
                    assert_eq!(
                        replaced.instrument, order.instrument,
                        "Snapshot<Order> does not have same instrument as existing Order<Open>"
                    );
                }
            }
            // Remove associated Open (expected), or InFlight (unexpected) order
            OrderState::Cancelled(_cancelled) => {
                if let Some(open) = self.opens.remove(&order.cid) {
                    debug!(
                        instrument = %order.instrument,
                        cid = %order.cid,
                        ?open,
                        cancel = ?order,
                        "OrderManager removed Order<Open> after receiving Snapshot<Order<Cancelled>>"
                    );
                } else if let Some(in_flight) = self.in_flights.remove(&order.cid) {
                    warn!(
                        instrument = %order.instrument,
                        cid = %order.cid,
                        ?in_flight,
                        cancel = ?order,
                        "OrderManager removed Order<InFlight> after receiving Snapshot<Order<Cancelled>> - why was this still InFlight?"
                    );
                } else {
                    warn!(
                        instrument = %order.instrument,
                        cid = %order.cid,
                        cancel = ?order,
                        "OrderManager ignoring Snapshot<Order<Cancelled> for un-tracked Order"
                    );
                }
            }
        }
    }
}

impl<InstrumentKey> Default for Orders<InstrumentKey> {
    fn default() -> Self {
        Self {
            in_flights: Default::default(),
            opens: Default::default(),
        }
    }
}
