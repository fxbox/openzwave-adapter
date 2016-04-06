#![allow(dead_code)]

extern crate transformable_channels_mock;

pub mod util {
    use std::marker::PhantomData;
    use std::hash::{ Hash, Hasher };

    #[derive(Debug, Clone)]
    pub struct Id<T> {
        phantom: PhantomData<T>,
    }

    impl<T> Id<T> {
        pub fn new(_: &str) -> Self {
            Id {
                phantom: PhantomData,
            }
        }
    }
    impl<T> PartialEq for Id<T> {
        fn eq(&self, _: &Self) -> bool {
            return true;
        }
    }
    impl<T> Eq for Id<T> {}

    impl<T> Hash for Id<T> {
        fn hash<H>(&self, _: &mut H) where H: Hasher {}
    }

    pub struct TagId;
}

pub mod services {
    use super::*;
    use std::collections::HashSet;

    #[derive(Clone)]
    pub struct Setter {
        pub kind: ChannelKind,
        pub updated: Option<values::TimeStamp>,
    }

    #[derive(Clone, Debug)]
    pub struct Getter {
        pub kind: ChannelKind,
        pub updated: Option<values::TimeStamp>,
    }

    #[derive(Clone)]
    pub struct AdapterId;

    #[derive(Clone)]
    pub struct ServiceId;
    pub struct Service;

    impl Service {
        pub fn empty(_: util::Id<ServiceId>, _: util::Id<AdapterId>) -> Service {
            Service
        }
    }

    pub trait IOMechanism {}

    pub struct Channel<IOMechanism> {
        pub tags: HashSet<util::Id<util::TagId>>,
        pub id: util::Id<IOMechanism>,
        pub service: util::Id<ServiceId>,
        pub mechanism: IOMechanism,
        pub adapter: util::Id<AdapterId>,
        pub last_seen: Option<values::TimeStamp>,
    }

    #[derive(Clone, Debug)]
    pub enum ChannelKind {
        Ready,
        OpenClosed
    }
}

pub mod values {
    pub enum Value {
        OpenClosed(OpenClosed),
        Unit
    }

    pub enum Type {
        OpenClosed
    }
    pub enum OpenClosed {
        Open,
        Closed
    }
    pub enum Range {}

    #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct TimeStamp;
}

pub mod api {
    use super::*;
    use std::collections::HashMap;
    use std::{ fmt, error };

    #[derive(Debug)]
    pub enum Error {
        InternalError(InternalError)
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "stubbed error")
        }
    }

    impl error::Error for Error {
        fn description(&self) -> &str {
            return "stubbed error"
        }
    }

    #[derive(Debug)]
    pub enum InternalError {
        NoSuchGetter(util::Id<services::Getter>)
    }
    pub struct User;
    pub type ResultMap<K, T, E> = HashMap<K, Result<T, E>>;
}

pub mod adapter {
    use transformable_channels_mock::mpsc::ExtSender;
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;

    pub trait AdapterManagerHandle {
        fn add_adapter(& self, adapter: Arc<Adapter>) -> Result<(), api::Error>;
        fn remove_adapter(& self, id: &util::Id<services::AdapterId>) -> Result<(), api::Error>;
        fn add_service(& self, service: services::Service) -> Result<(), api::Error>;
        fn remove_service(& self, service_id: &util::Id<services::ServiceId>) -> Result<(), api::Error>;
        fn add_getter(& self, setter: services::Channel<services::Getter>) -> Result<(), api::Error>;
        fn remove_getter(& self, id: &util::Id<services::Getter>) -> Result<(), api::Error>;
        fn add_setter(& self, setter: services::Channel<services::Setter>) -> Result<(), api::Error>;
        fn remove_setter(& self, id: &util::Id<services::Setter>) -> Result<(), api::Error>;
    }

    pub struct AdapterManagerHandleImpl;
    impl AdapterManagerHandle for AdapterManagerHandleImpl {
        fn add_adapter(& self, _: Arc<Adapter>) -> Result<(), api::Error> {
            Ok(())
        }
        fn remove_adapter(& self, _: &util::Id<services::AdapterId>) -> Result<(), api::Error> {
            Ok(())
        }
        fn add_service(& self, _: services::Service) -> Result<(), api::Error> {
            Ok(())
        }
        fn remove_service(& self, _: &util::Id<services::ServiceId>) -> Result<(), api::Error> {
            Ok(())
        }
        fn add_getter(& self, _: services::Channel<services::Getter>) -> Result<(), api::Error> {
            Ok(())
        }
        fn remove_getter(& self, _: &util::Id<services::Getter>) -> Result<(), api::Error> {
            Ok(())
        }
        fn add_setter(& self, _: services::Channel<services::Setter>) -> Result<(), api::Error> {
            Ok(())
        }
        fn remove_setter(& self, _: &util::Id<services::Setter>) -> Result<(), api::Error> {
            Ok(())
        }
    }

    pub trait AdapterWatchGuard {}

    pub trait Adapter {
        fn id(&self) -> super::util::Id<super::services::AdapterId>;
        fn name(&self) -> &str;
        fn vendor(&self) -> &str;
        fn version(&self) -> &[u32; 4];
        fn fetch_values(&self, mut set: Vec<util::Id<services::Getter>>, _: api::User) -> api::ResultMap<util::Id<services::Getter>, Option<values::Value>, api::Error>;
        fn send_values(&self, values: HashMap<util::Id<services::Setter>, values::Value>, _: api::User) -> api::ResultMap<util::Id<services::Setter>, (), api::Error>;
        fn register_watch(&self, mut values: Vec<(util::Id<services::Getter>, Option<values::Range>)>, sender: Box<ExtSender<WatchEvent>>) -> api::ResultMap<util::Id<services::Getter>, Box<AdapterWatchGuard>, api::Error>;
    }
    pub enum WatchEvent {
        Enter {
            id: util::Id<services::Getter>,
            value: values::Value
        },

        Exit {
            id: util::Id<services::Getter>,
            value: values::Value
        }
    }
}
