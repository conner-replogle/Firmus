mod communication;
pub mod metrics;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        metrics::initialize();
        
    }
}
