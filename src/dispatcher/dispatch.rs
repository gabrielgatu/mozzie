use intent::Intent;
use context::Context;
use service::{Service, Outcome, Error};

/// Takes an intent and tries to find a service that can handle it.
/// The service search is divided into 4 levels. The first level that returns a valid
/// service is picked:
/// 
/// # SERVICES THAT CAN HANDLE THE INTENT
/// 1) It searches the service inside the history of the recently used services,
/// that can handle the intent, as it is.
/// 2) It searches the service into the list of services available.
/// 
/// # SERVICES THAT CAN HANDLE THE CALL
/// 3) It searches the service inside the history of the recently used services,
/// that can handle the call with the params from the intent.
/// 4) It searches the services that can handle the call, from the list of services available.
pub fn dispatch_intent(intent: Intent) -> Outcome {
  match get_service_for_intent(&intent) {
    Some(service) => service.handle_action(&intent),
    None => Outcome::Failure(Error::ServiceNotFound),
  }
}

// Really need to refactor this :)
fn get_service_for_intent<'a>(intent: &Intent) -> Option<&'a Box<Service>> {
  find_service_satisfying_intent(Context::history_stack(), intent).or_else(|| {
    find_service_satisfying_intent(Context::services(), intent).or_else(|| {
      find_service_satisfying_action_call(Context::history_stack(), intent).or_else(|| {
        find_service_satisfying_action_call(Context::services(), intent)
      })
    })
  })
}

fn find_service_satisfying_intent<'a, T: Service + ?Sized>(services: &'a Vec<Box<T>>, intent: &Intent) -> Option<&'a Box<T>> {
  services
    .into_iter()
    .filter(|service| {
      (service.categories()[..]).contains(&intent.category)
    })
    .nth(0)
}

fn find_service_satisfying_action_call<'a, T: Service + ?Sized>(services: &'a Vec<Box<T>>, intent: &Intent) -> Option<&'a Box<T>> {
  services
    .into_iter()
    .filter(|service| {
      service.can_handle() // TODO: Pass intent
    })
    .nth(0)
}