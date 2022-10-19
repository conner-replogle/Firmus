pub struct DashConnection{
    url: String,
}
impl DashConnection{
    pub fn new(url: String) -> Result<Self,()>{
        Ok(Self{
            url
        })
    }
}