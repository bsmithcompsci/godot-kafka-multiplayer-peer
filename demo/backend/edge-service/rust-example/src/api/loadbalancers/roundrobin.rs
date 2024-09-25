use crate::api::loadbalancer::LoadBalancer;

pub struct RoundRobin {
    servers: Vec<super::super::server::Server>,
    current: i32,
}

impl LoadBalancer for RoundRobin {
    fn next(&mut self) -> &mut crate::api::server::Server {
        self.current = (self.current + 1) % self.servers.len() as i32;
        &mut self.servers[self.current as usize]
    }
}