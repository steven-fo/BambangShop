use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::{self, Subscriber};

lazy_static! {
    static ref SUBSRICBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSRICBERS.get(product_type).is_none() {
            SUBSRICBERS.insert(String::from(product_type), DashMap::new());
        };

        SUBSRICBERS.get(product_type).unwrap().insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }
}