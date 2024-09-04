use crate::api::{loadbalancer::LoadBalancer, server::Server};

pub struct LeastConnections {
    servers: Vec<Server>
}

impl LoadBalancer for LeastConnections {
    fn next(&mut self) -> &mut Server {
        let mut min = usize::MAX;
        let mut index = 0;
        for (i, server) in self.servers.iter().enumerate() {
            if server.get_clients().len() < min {
                min = server.get_clients().len();
                index = i;
            }
        }
        &mut self.servers[index]
    }
}