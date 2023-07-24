use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_ticket: Semaphore,
}
#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}

impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}

impl Museum {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_ticket: Semaphore::new(total),
        }
    }

    // 他是可以在這裡自行推斷說生命週期應該要跟 傳進來的&self 他的生命週期一樣，所以才可以省略只使用佔位符'_'
    // 不使用的話像這樣 pub fn get_ticket<'a>(&'a self) -> Option<Ticket<'a>> {}

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
        match self.remaining_ticket.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn ticket(&self) -> usize {
        self.remaining_ticket.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let museum = Museum::new(60);
        assert_eq!(museum.ticket(), 60);
        let ticket = museum.get_ticket().unwrap();
        println!("{:?}", ticket);
        let tickets: Vec<Ticket> = (0..59).map(|_| museum.get_ticket().unwrap()).collect();
        assert_eq!(museum.ticket(), 0);
        assert!(museum.get_ticket().is_none());
    }
}
