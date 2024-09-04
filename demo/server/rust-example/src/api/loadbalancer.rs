pub trait LoadBalancer {
    fn next(&mut self) -> &mut super::server::Server;
}