use std::ops::Not;

type Result<T> = anyhow::Result<T, anyhow::Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeApiKind {
    Ews,
    Graph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServerKind {
    Exchange2016(Vec<ExchangeApiKind>),
    Exchange2019(Vec<ExchangeApiKind>),
    ExchangeOnline(Vec<ExchangeApiKind>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeServer {
    OnPremise(ExchangeServerKind),
    Online(ExchangeServerKind),
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum ExchangeServerState {
    Connected,
    Authenticated,
    Unauthenticated,
    #[default]
    Disconnected,
}

pub struct ExchangeConnection<ServerType = ExchangeServer, ServerState = ExchangeServerState> {
    server_type: ServerType,
    server_state: ServerState,
}

impl ExchangeConnection {
    pub fn new(server_type: ExchangeServer, server_state: ExchangeServerState) -> Self {
        Self {
            server_type,
            server_state,
        }
    }

    pub fn connected(&self) {
        self.server_state == ExchangeServerState::Connected
    }

    pub fn disconnected(&self) {
        self.connected().not()
    }

    pub fn authenticated(&self) {
        self.server_state == ExchangeServerState::Authenticated
    }

    pub fn unauthenticated(&self) {
        self.authenticated().not()
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

    pub fn is_ews(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2016(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2019(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            ExchangeServer::Online(ExchangeServerKind::ExchangeOnline(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Ews)
            }
            _ => false,
        }
    }

    pub fn is_graph(&self) -> bool {
        match self.server_type {
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2016(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            ExchangeServer::OnPremise(ExchangeServerKind::Exchange2019(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            ExchangeServer::Online(ExchangeServerKind::ExchangeOnline(ref api)) => {
                api.iter().any(|api| api == &ExchangeApiKind::Graph)
            }
            _ => false,
        }
    }
}
