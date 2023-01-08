use super::AOCSolutions; 

type Item = usize; 

struct Monkey {
    ident: usize, 
    items: Vec<Item>,

    /**
    Operation to be performed when the monkey inspects an item in its inventory.
     */
    op: fn(Item) -> Item, 

    // Operation to determine communication between monkes will be encased in a wrapping structure, 
    // for ownership safety. 
}

impl Monkey {
    pub fn new(ident: usize, items: Vec<usize>, op: fn(Item) -> Item) -> Monkey {
        Monkey { ident, items, op }
    }

    pub fn inspect_item(&mut self) -> Option<Item> {
        match self.items.pop() {
            Some(item) => Some((self.op)(item)), 
            None => None, 
        }
    }

    pub fn receive_item(&mut self, item: Item) {
        self.items.push(item); 
    }
}

struct MonkeyBusiness {
    monkeys: Vec<Monkey>,
    
    /**
    Operation to be performed to determine to which `Monkey` a `Monkey` is allowed to pass an `Item` 
    to.
     */
    test: Box<dyn Fn(&Monkey, Item) -> &mut Monkey>, 
}

impl MonkeyBusiness {
    pub fn play_round(&mut self) {
        for monkey in self.monkeys.iter_mut() {
            while let Some(item) = monkey.inspect_item() {
                let recipient = (self.test)(monkey, item);
                // :? Can this even pass borrow check? `recepient` may point to `monkey` which is 2 `&mut` to one thing in one scope!
                recipient.receive_item(item); 
            }
        }
    }
}





