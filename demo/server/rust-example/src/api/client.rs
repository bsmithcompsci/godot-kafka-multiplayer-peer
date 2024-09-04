pub struct Client
{
    id: i64,
}

impl Client
{
    pub fn new(id: i64) -> Client
    {
        Client
        {
            id,
        }
    }

    pub fn get_id(&self) -> i64
    {
        self.id
    }
}