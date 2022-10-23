use crate::stream_wrapper::Stream;
#[derive(serde::Serialize,serde::Deserialize)]
pub struct ProcessStatus{
    id: String,
    ipc_online: bool,
    process_online: bool,
    pid: String,


}

pub struct Process{
    pub handle: Handle
    pub stream: Option<Stream>
}
impl Process{
    pub fn start({
        
    })
    pub fn into_status(&self)-> ProcessStatus{
        ProcessStatus{
            id: "".to_string()  ,
            ipc_online: self.stream.is_some(),
            process_online: self.stream.is_some(),
            pid: "".to_string(),
        }
        

    }
}