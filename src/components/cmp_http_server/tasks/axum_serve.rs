use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::routing;

pub struct AxumServe {
    pub port: u16,
    pub router: routing::Router,
}

impl AxumServe {
    pub async fn spawn(self) -> super::Result<()> {
        let ipaddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let socket_addr = SocketAddr::new(ipaddr, self.port);

        let listener = tokio::net::TcpListener::bind(socket_addr)
            .await
            .map_err(super::Error::BindPort)?;

        axum::serve(listener, self.router)
            .await
            .map_err(super::Error::AxumServe)?;

        Err(super::Error::TaskEndAxumServe)
    }
}
