use std::sync::LazyLock;
pub type Fetcher = reqwest::Client;

pub static ON_AIR: LazyLock<Fetcher> = LazyLock::new(|| {
    let config = &config::get().fetcher;
    crate_fetcher(&config.clients.onair, config.proxy.get_uri())
});

pub static BANGUMI: LazyLock<Fetcher> = LazyLock::new(|| {
    let config = &config::get().fetcher;
    crate_fetcher(&config.clients.bangumi, config.proxy.get_uri())
});

pub fn get_onair() -> &'static Fetcher {
    &ON_AIR
}

pub fn get_bangumi() -> &'static Fetcher {
    &BANGUMI
}

pub fn crate_fetcher(config: &config::fetcher::Fetcher, proxy: Option<String>) -> Fetcher {
    let mut client_builder = reqwest::ClientBuilder::new();
    if config.use_proxy {
        if let Some(proxy) = proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::all(proxy).unwrap());
        }
    }
    if let Some(max_conn) = config.max_conn {
        client_builder = client_builder.pool_max_idle_per_host(max_conn);
    }
    if let Some(timeout_secs) = config.timeout_secs {
        client_builder = client_builder.timeout(std::time::Duration::from_secs(timeout_secs));
    }
    if let Some(headers) = &config.headers {
        client_builder = client_builder.default_headers(
            headers
                .into_iter()
                .filter_map(|(k, v)| {
                    Some((
                        reqwest::header::HeaderName::from_bytes(k.as_bytes()).ok()?,
                        reqwest::header::HeaderValue::from_str(&v).ok()?,
                    ))
                })
                .collect(),
        );
    }
    client_builder.build().expect("Failed to initialize config")
}
