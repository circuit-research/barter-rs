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
use uuid::Uuid;
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
                        // notification, so don't modify with Open response data (could be stale)
                        warn!(
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
            (Some(in_flight), Err(error)) => {
                warn!(
                    instrument = %response.instrument,
                    cid = %response.cid,
                    ?in_flight,
                    ?error,
                    "OrderManager received ExecutionError for Order<InFlight>"
                );
            }
            (None, Ok(open)) => {
                warn!(
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
                        warn!(
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
            (None, Err(error)) => {
                error!(
                    instrument = %response.instrument,
                    cid = %response.cid,
                    ?error,
                    "OrderManager received ExecutionError for non-existing Order<InFlight>"
                );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::order::OrderId;
    use barter_integration::model::Side;

    fn in_flights(
        orders: impl IntoIterator<Item = Order<u64, InFlight>>,
    ) -> VecMap<ClientOrderId, Order<u64, InFlight>> {
        orders.into_iter().map(|order| (order.cid, order)).collect()
    }

    fn in_flight(cid: ClientOrderId) -> Order<u64, InFlight> {
        Order {
            instrument: 1,
            cid,
            side: Side::Buy,
            state: InFlight,
        }
    }

    fn opens(
        orders: impl IntoIterator<Item = Order<u64, Open>>,
    ) -> VecMap<ClientOrderId, Order<u64, Open>> {
        orders.into_iter().map(|order| (order.cid, order)).collect()
    }

    fn open(cid: ClientOrderId, id: OrderId) -> Order<u64, Open> {
        Order {
            instrument: 1,
            cid,
            side: Side::Buy,
            state: Open {
                id,
                time_update: Default::default(),
                price: 0.0,
                quantity: 0.0,
                filled_quantity: 0.0,
            },
        }
    }

    fn open_ok(cid: ClientOrderId, id: OrderId) -> Order<u64, Result<Open, ExecutionError>> {
        Order {
            instrument: 1,
            cid,
            side: Side::Buy,
            state: Ok(Open {
                id,
                time_update: Default::default(),
                price: 0.0,
                quantity: 0.0,
                filled_quantity: 0.0,
            }),
        }
    }

    fn open_err(cid: ClientOrderId) -> Order<u64, Result<Open, ExecutionError>> {
        Order {
            instrument: 1,
            cid,
            side: Side::Buy,
            state: Err(ExecutionError::X),
        }
    }

    #[test]
    fn test_record_in_flights() {
        struct TestCase {
            state: Orders<u64>,
            input: Vec<Order<u64, InFlight>>,
            expected: Orders<u64>,
        }

        let cid_1 = ClientOrderId(Uuid::new_v4());

        let cases = vec![
            TestCase {
                // TC0: Insert unseen InFlight
                state: Orders {
                    in_flights: Default::default(),
                    opens: Default::default(),
                },
                input: vec![in_flight(cid_1)],
                expected: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: Default::default(),
                },
            },
            TestCase {
                // TC1: Insert InFlight that is already tracked
                state: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: Default::default(),
                },
                input: vec![],
                expected: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: Default::default(),
                },
            },
        ];

        for (index, mut test) in cases.into_iter().enumerate() {
            test.state.record_in_flights(test.input);
            assert_eq!(test.state, test.expected, "TestCase {index} failed")
        }
    }

    #[test]
    fn test_update_from_open() {
        struct TestCase {
            state: Orders<u64>,
            input: Order<u64, Result<Open, ExecutionError>>,
            expected: Orders<u64>,
        }

        let cid_1 = ClientOrderId(Uuid::new_v4());
        let order_id_1 = OrderId::new("order_id_1".to_string());

        let cases = vec![
            TestCase {
                // TC0: InFlight present, response Ok(open), Open present
                state: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
                input: open_ok(cid_1, order_id_1.clone()),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC1: InFlight present, response Ok(open), Open not-present
                state: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: Default::default(),
                },
                input: open_ok(cid_1, order_id_1.clone()),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC2: InFlight present, response Err(open), Open not-present
                state: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: Default::default(),
                },
                input: open_err(cid_1),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: Default::default(),
                },
            },
            TestCase {
                // TC3: InFlight present, response Err(open), Open present
                state: Orders {
                    in_flights: in_flights([in_flight(cid_1)]),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
                input: open_err(cid_1),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC4: InFlight not-present, response Ok(open), Open present
                state: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
                input: open_ok(cid_1, order_id_1.clone()),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC5: InFlight not-present, response Ok(open), Open not-present
                state: Orders {
                    in_flights: Default::default(),
                    opens: Default::default(),
                },
                input: open_ok(cid_1, order_id_1.clone()),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC6: InFlight not-present, response Err(open), Open present
                state: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
                input: open_err(cid_1),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: opens([open(cid_1, order_id_1.clone())]),
                },
            },
            TestCase {
                // TC7: InFlight not-present, response Err(open), Open not-present
                state: Orders {
                    in_flights: Default::default(),
                    opens: Default::default(),
                },
                input: open_err(cid_1),
                expected: Orders {
                    in_flights: Default::default(),
                    opens: Default::default(),
                },
            },
        ];

        for (index, mut test) in cases.into_iter().enumerate() {
            test.state.update_from_open(&test.input);
            assert_eq!(test.state, test.expected, "TestCase {index} failed")
        }
    }

    #[test]
    fn test_update_from_cancel() {
        todo!()

        // Todo: update these scenarios, they are from update_from_open
        // Scenarios:
        // - InFlight present, Open not-present, response Ok(open)
        // - InFlight present, Open not-present, response Err(open)

        // - InFlight present, Open present, response Ok(open)
        // - InFlight present, Open present, response Err(open)

        // - InFlight not-present, Open not-present, response Ok(open)
        // - InFlight not-present, Open present, response Err(open)
    }

    #[test]
    fn test_update_from_order_snapshot() {
        todo!()
    }
}
