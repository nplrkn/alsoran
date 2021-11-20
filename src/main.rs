mod gnbcu;
mod logging;
#[cfg(test)]
mod mock_transport_provider;
mod transport_provider;

#[async_std::main]
async fn main() {
    let _logger = logging::init();
}
