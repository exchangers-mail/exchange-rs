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

impl ExchangeConnection {
    pub fn new(server_type: ExchangeServer) -> Self {
        Self {
            server_type,
            connected: false,
        }
    }

    pub fn is_onpremise(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(_) => true,
            _ => false,
        }
    }

    pub fn is_online(&self) -> bool {
        match self.server_type {
            ExchangeServer::Online(_) => true,
            _ => false,
        }
    }
}
