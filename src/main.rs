use std::io;
use std::mem;

struct Node<T>{
    data:T,
    next:Option<Box<Node<T>>>
}
impl <T> Node<T> where T:std::fmt::Display{
    fn new(data:T) -> Box<Node<T>>{
        Box::new(Node{
            data,
            next : None
        })
    }

    fn ins_f(&mut self, value:T){
        let old_next = self.next.take();
        let old_data = mem::replace(&mut self.data,value);
         let nn = Box::new(Node{
             data:old_data,
             next : old_next
         });
        self.next = Some(nn);
    }

    fn ins_l(&mut self, value:T){
        let mut  t =self;
        while t.next.is_some(){
            t = t.next.as_deref_mut().unwrap();
        }
        t.next = Some( Node::new(value));
    }

    fn del_f(&mut self){
        match self.next.take(){
            Some(node) => {
                let _ = mem::replace(&mut self.data, node.data);
                self.next = node.next;

            }
            None => return
        }
    }

    fn del_l(&mut self){
        let mut t = self;
        while t.next.as_deref().map_or(false, |node| node.next.is_some()){
            t = t.next.as_deref_mut().unwrap();
        }
        t.next = None;
    }

    fn display(&self) {
        let mut t = self;
        loop{
            print!("{} -> ", t.data);
            if t.next.is_none(){
                break;
            }
            t = t.next.as_deref().unwrap();

        }
    }

}



fn main() {
    print!("enter the first element of the list");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("unable to read");
    let value:i32 = value.trim().parse().expect("enter a integer");
    let mut head = Box::new(Node::new(value));
    loop{
        let mut op = String::new();
        println!("1.ins first");
        println!("2.ins last");
        println!("3.del first");
        println!("4.del last");
        println!("5.display");
        println!("6.exit");
        io::stdin()
            .read_line(&mut op)
            .expect("unable to read");
        let op:i32 = op.trim().parse().expect("enter a integer");
        match op{
            1 =>{
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("unable to read");
                let value:i32 = value.trim().parse().expect("enter a integer");
                head.ins_f(value);
            },
            2=>{
                let mut value = String::new();
                io::stdin()
                    .read_line(&mut value)
                    .expect("unable to read");
                let value:i32 = value.trim().parse().expect("enter a integer");
                head.ins_l(value);
            }
            3=>head.del_f(),

            4=> head.del_l(),
            5=> head.display(),
            6=> break ,
            _ => println!("enter a valid option")
        }

    }


}




























