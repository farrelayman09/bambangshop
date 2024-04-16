use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subcriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
}