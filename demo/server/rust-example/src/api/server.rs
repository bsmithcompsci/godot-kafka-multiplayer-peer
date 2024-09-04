pub struct Server
{
    id: i64,
    clients: Vec<super::client::Client>,
}

impl Server
{
    pub fn new(id: i64) -> Server
    {
        Server
        {
            id,
            clients: Vec::new(),
        }
    }

    pub fn get_id(&self) -> i64
    {
        self.id
    }

    pub fn add_client(&mut self, client: super::client::Client)
    {
        self.clients.push(client);
    }

    pub fn get_clients(&self) -> &Vec<super::client::Client>
    {
        &self.clients
    }
    pub fn remove_client(&mut self, client_id: i64)
    {
        self.clients.retain(|client| client.get_id() != client_id);
    }

    pub fn get_client(&self, client_id: i64) -> Option<&super::client::Client>
    {
        self.clients.iter().find(|client| client.get_id() == client_id)
    }
}