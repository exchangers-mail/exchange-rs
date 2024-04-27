pub enum ExchangeApiKind {
    Ews,
    Graph,
}

pub enum ExchangeServerKind {
    2016(Vec<ExchangeApiKind>),
    2019(Vec<ExchangeApiKind>),
    Online(ExchangeApiKind::Graph),
}

pub enum ExchangeServer {
    OnPremise(ExchangeServerKind),
    Online(ExchangeServerKind::Online),
}

pub struct<T = ExchangeServer> ExchangeConnection<T> {
    server_type: T,
    connected: bool,
}

impl ExchangeConnection<Target = ExchangeServer> {
    fn is_online(&self) -> bool;
    fn is_connected(&self) -> bool;
    fn is_cloud(&self) -> bool {
        self.is_online()
    }
    fn is_on_premise(&self) -> bool {
        !self.is_online()
    }
    fn is_onpremise(&self) -> bool {
        self.is_on_premise()
    }

    fn raw(&self) -> Option<String> {
        None
    }
    fn is_parsed(&self) -> bool {
        false
    }
    fn parse(&self) -> Option<String> {
        None
    }
}
