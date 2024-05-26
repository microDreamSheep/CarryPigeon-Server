pub trait GroupMessageService {}

pub trait PrivateMessageService {}

pub trait SystemMessageService {}

pub struct MessageService;

impl GroupMessageService for MessageService {}

impl PrivateMessageService for MessageService {}

impl SystemMessageService for MessageService {}
