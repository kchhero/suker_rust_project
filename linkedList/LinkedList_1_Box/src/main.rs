struct ListItem<T> {
    data: Box<T>,   // 데이터는 각 리스트 항목마다 박스된다. 데이터 필드는 비어 있거나 널일 수 없다.
    next: Option<Box<ListItem<T>>>, //next 포인터는 선택 사항이다. 리스트에 후속 요소가 있는지 알 수 없으므로 Option에 Box를 놓고, 리스트의 다음 항목을 가리킨다.
}

struct SinglyLinkedList<T> {
    head: ListItem<T>, //리스트 자체의 구조체에는 head만 포함되어있다. head는 항상 존재해야하므로 박스하지 않는다.
}

impl<T> ListItem<T> {
    fn new(data: T) -> Self {
        ListItem {
            data: Box::new(data), //새 데이터는 Box에 넣어 새로운 리스트 항목으로 이동한다.
            //데이터는 힙에 할당되며 스택에서 힙으로 데이터를 이동해야 할 수 있으므로 컴파일러는 데이터를
            //대상 위치로 가져오는 데 필요한 세부 정보를 정렬한다.
            next: None, //next포인터는 새로운 요소가 목록의 어디에 있는지 아직 모르기 때문에 None으로 초기화된다.
            //또한 이 구형에는 삽입 작업이 없고 추가만 있다.
        }
    }
    fn next(&self) -> Option<&Self> { //각 항목의 next() 메서드는 다음 항목에 대한 선택적 참조를 반환한다. 이함수는 중첩된 참조를 풀어 코드를 단순화하기 위해 존재한다.
        if let Some(next) = &self.next { //if let ... 구문으로 역참조를 시도하기 전에 next 포인터가 가리키는 항목이 있는지 확인한다.
            Some(next.as_ref()) //next 항목에 대한 내부 참조를 반환했다. Some(&*next)와 동일하다.
        } else {
            None
        }
    }
    fn mut_tail(&mut self) -> &mut Self {
        if self.next.is_some() { //self.netxt를 빌려야 하기 때문에 여기서는 if let... 구문을 사용할 수 없다.
            self.next.as_mut().unwrap().mut_tail() //Box를 Option으로 감싸고 있으므로 Option을 가변 참조로부터 풀어야하고 그 안의 가변 참조를 반환한다.
        } else {
            self
        }
    }
    fn data(&self) -> &T {
        self.data.as_ref() //T에 직접 엑세스하는 편의를 위함.
    }
}

impl<T> SinglyLinkedList<T> {
    fn new(data: T) -> Self {
        SinglyLinkedList {
            head: ListItem::new(data), //새 리스트를 만들려면 첫 번째 요소가 필요하다.
        }
    }
    fn append(&mut self, data: T) {
        let mut tail = self.head.mut_tail(); //새 요소를 추가할 때 꼬리의 다음 항목은 None으로 가정한다.
        tail.next = Some(Box::new(ListItem::new(data))); //새 요소를 꼬리 항목의 다음 포인터에 추가하면 새 요소가 새꼬리가 된다.
    }
    fn head(&self) -> &ListItem<T> {
        &self.head //편의상 헤드 요소를 가져오는 메서드를 만들었다.
    }
}

fn main() {
    let mut list = SinglyLinkedList::new(1); //새 단일 연결 리스트를 만든다.
    list.append(2); //리스트에 요소를 추가한다.
    list.append(3);
    list.append(4);

    let mut current = list.head(); //헤드 요소에서 시작한다.
    loop {
        println!("item: {}", current.data()); //현재 요소의 데이터를 출력한다.
        if let Some(next) = current.next() { //다음 요소가 있는지 확인한다.
            current = next; //다음 요소로 이동한다.
        } else {
            break; //다음 요소가 없으면 루프를 종료한다.
        }
    }
}
