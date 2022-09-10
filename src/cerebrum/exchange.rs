

// Todo: May need to have an synchronous interface prior to async for eg/ GenerateClientOrderId
pub enum ExchangeCommand {
    // Check connection status
    Status,

    // Fetch Account State
    FetchOpenOrders,
    FetchBalances,

    // Open Orders
    OpenOrder,
    OpenOrderBatch,

    // Cancel Orders
    CancelOrderById,
    CancelOrderByInstrument,
    CancelOrderByBatch,
    CancelOrderAll,
}