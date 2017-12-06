use dispatcher::HistoryStack;
use service::Service;
use service::Multimedia;

lazy_static! {
    static ref CONTEXT: Context = Context::new();
}

pub struct Context {
  history_stack: HistoryStack<Box<Service>>,
  services: Vec<Box<Service>>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      history_stack: HistoryStack::new(),
      services: vec![Box::new(Multimedia::new())],
    }
  }

  pub fn get() -> &'static Context {
    &CONTEXT
  }

  pub fn history_stack<'a>() -> &'a Vec<Box<Service>> {
    Context::get()
      .history_stack
      .all()
  }

  pub fn services<'a>() -> &'a Vec<Box<Service>> {
    &Context::get().services
  }
}
