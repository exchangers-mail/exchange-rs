pub enum ExchangeApiKind {
    Ews,
    Graph,
}

pub enum ExchangeServerKind {
    Exchange2016(Vec<ExchangeApiKind>),
    Exchange2019(Vec<ExchangeApiKind>),
    ExchangeOnline(ExchangeApiKind),
}

pub enum ExchangeServer {
    OnPremise(ExchangeServerKind),
    Online(ExchangeServerKind),
}

pub struct ExchangeConnection<T = ExchangeServer> {
    server_type: T,
    connected: bool,
}
