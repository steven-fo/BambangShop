use dashmap::Dashmap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

lazy_static! {
    static ref SUBSRICBERS: Dashmap<String, Dashmap<String, Subscriber>> = Dashmap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    
}