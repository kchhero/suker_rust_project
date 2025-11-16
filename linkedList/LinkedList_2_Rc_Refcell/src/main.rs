use std::cell::RefCell;
use std::rc::Rc;

struct ListItem<T> {
    //data: Box<T>,   // 데이터는 각 리스트 항목마다 박스된다. 데이터 필드는 비어 있거나 널일 수 없다.
    //next: Option<Box<ListItem<T>>>, //next 포인터는 선택 사항이다. 리스트에 후속 요소가 있는지 알 수 없으므로 Option에 Box를 놓고, 리스트의 다음 항목을 가리킨다.
    prev: Option<ItemRef<T>>, //리스트의 이전 항목에 대한 포인터를 추가했다.
    data: Box<T>,   // 데이터는 여전히 Box에 보관된다. 데이터 소유권을 공유하지 않고 리스트의 
                    // 노드에 대한 포인터만 공유하기 때문에 여기서는 Rc를 사용할 필요가 없다.
    next: Option<ItemRef<T>>,
}
type ItemRef<T> = Rc<RefCell<ListItem<T>>>; //코드를 깔끔하게 유지하기 위한 typedef
// struct SinglyLinkedList<T> {
//     head: ListItem<T>, //리스트 자체의 구조체에는 head만 포함되어있다. head는 항상 존재해야하므로 박스하지 않는다.
// }
struct DoublyLinkedList<T> {
    head: ItemRef<T>, //리스트의 헤드는 이제 선택 사항이다. 리스트가 비어 있을 수 있기 때문이다. 
}
impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            // data: Box::new(data), //새 데이터는 Box에 넣어 새로운 리스트 항목으로 이동한다.
            // //데이터는 힙에 할당되며 스택에서 힙으로 데이터를 이동해야 할 수 있으므로 컴파일러는 데이터를
            // //대상 위치로 가져오는 데 필요한 세부 정보를 정렬한다.
            // next: None, //next포인터는 새로운 요소가 목록의 어디에 있는지 아직 모르기 때문에 None으로 초기화된다.
            // //또한 이 구형에는 삽입 작업이 없고 추가만 있다.
            prev: None,
            data: Box::new(data), //데이터가 Box로 이동된다.
            next: None,
        }
    }
    // fn next(&self) -> Option<&Self> { //각 항목의 next() 메서드는 다음 항목에 대한 선택적 참조를 반환한다. 이함수는 중첩된 참조를 풀어 코드를 단순화하기 위해 존재한다.
    //     if let Some(next) = &self.next { //if let ... 구문으로 역참조를 시도하기 전에 next 포인터가 가리키는 항목이 있는지 확인한다.
    //         Some(next.as_ref()) //next 항목에 대한 내부 참조를 반환했다. Some(&*next)와 동일하다.
    //     } else {
    //         None
    //     }
    // }
    // fn mut_tail(&mut self) -> &mut Self {
    //     if self.next.is_some() { //self.netxt를 빌려야 하기 때문에 여기서는 if let... 구문을 사용할 수 없다.
    //         self.next.as_mut().unwrap().mut_tail() //Box를 Option으로 감싸고 있으므로 Option을 가변 참조로부터 풀어야하고 그 안의 가변 참조를 반환한다.
    //     } else {
    //         self
    //     }
    // }
    fn data(&self) -> &T {
        self.data.as_ref() //T에 직접 엑세스하는 편의를 위함.
    }
}

// impl<T> SinglyLinkedList<T> {
//     fn new(data: T) -> Self {
//         SinglyLinkedList {
//             head: ListItem::new(data), //새 리스트를 만들려면 첫 번째 요소가 필요하다.
//         }
//     }
//     fn append(&mut self, data: T) {
//         let mut tail = self.head.mut_tail(); //새 요소를 추가할 때 꼬리의 다음 항목은 None으로 가정한다.
//         tail.next = Some(Box::new(ListItem::new(data))); //새 요소를 꼬리 항목의 다음 포인터에 추가하면 새 요소가 새꼬리가 된다.
//     }
//     fn head(&self) -> &ListItem<T> {
//         &self.head //편의상 헤드 요소를 가져오는 메서드를 만들었다.
//     }
// }
impl<T> DoublyLinkedList<T> {
    fn new(data: T) -> Self {
        DoublyLinkedList {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }
    fn append(&mut self, data: T) {
        let tail = Self::find_tail(self.head.clone()); //먼저 리스트에서 꼬리 항목에 대한 포인터를 찾아야한다.
        let new_item = Rc::new(RefCell::new(ListItem::new(data))); //새 항목을 만든다.
        new_item.borrow_mut().prev = Some(tail.clone()); //이전 꼬리를 가리키도록 새 항목의 prev 포인터를 업데이트한다.
        tail.borrow_mut().next = Some(new_item); //새로 삽입된 항목이 새로운 꼬리가 되도록 이전 꼬리의 next 포인터를 업데이트한다.
    }
    fn head(&self) -> ItemRef<T> {
        self.head.clone()
    }
    fn tail(&self) -> ItemRef<T> {
        Self::find_tail(self.head.clone())
    }
    fn find_tail(item: ItemRef<T>) -> ItemRef<T> {
        if let Some(next) = &item.borrow().next { //next 포인터가 비어있는지 확인하고 그렇지 않으면 재귀적으로 검색을 계속한다.
            Self::find_tail(next.clone()) //다음 포인터를 복제하고 반환하여 재귀적으로 검색을 계속한다.
        } else {
            item.clone() //next 포인터가 비어 있으면 목록의 끝에 온것이다. 현재 항목을 반환한다.
        }
    }
}

fn main() {
    let mut list = DoublyLinkedList::new(1);
    list.append(2);
    list.append(5);
    list.append(4);
    list.append(3);

    let head = list.head();
    let tail = list.tail();

    println!("Head data: {}", head.borrow().data());
    println!("Tail data: {}", tail.borrow().data());
}
