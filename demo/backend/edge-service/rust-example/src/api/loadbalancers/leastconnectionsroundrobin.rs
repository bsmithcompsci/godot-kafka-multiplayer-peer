// Least Connection Rounded Robin Load Balancer
//  This load balancer selects the server with the least number of active connections.
//  This is done by iterating through the servers, finding the average number of clients and round robin selecting the servers below the average.

use crate::api::{loadbalancer::LoadBalancer, server::Server};

pub struct LeastConnectionsRoundRobin {
    servers: Vec<Server>,
    current: i32,
}

impl LoadBalancer for LeastConnectionsRoundRobin {
    fn next(&mut self) -> &mut Server {
        let mut average: i32 = 0;
        {
            let mut total_clients: i32 = 0;
            for server in self.servers.iter() {
                let clients = server.get_clients().len() as i32;
                total_clients += clients;
                average += clients;
            }
            
            average /= total_clients;
        }

        let mut index = 0;
        for _ in 0..self.servers.len() {
            self.current = (self.current + 1) % self.servers.len() as i32;
            let clients = self.servers[self.current as usize].get_clients().len() as i32;
            if clients < average {
                index = self.current;
                break;
            }
        }

        &mut self.servers[index as usize]
    }
}