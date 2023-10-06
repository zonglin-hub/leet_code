# _23_merge_k_lists


## 解题思路


这道题目的解题思路是使用最小堆来合并 K 个有序链表，将 K 个链表的首节点存入一个最小堆中，然后每次从最小堆中取出最小的节点作为合并后链表的下一个节点，再将该节点的下一个节点插入最小堆中。这样，在每次取出最小节点并将其放到合并后的链表中后，最小堆中一定会有 K 个元素（除非 K 个链表都遍历完了）。

因为最小堆会优先弹出堆顶元素，所以合并后的链表就是按照升序排列的。在实现中，我们可以使用 std::collections::BinaryHeap 来表示最小堆。

具体的实现细节如下：

首先将 K 个链表的头节点插入最小堆中。

每次从最小堆中取出堆顶元素（即最小的链表头节点），将其放入合并后的链表中，并将其下一个节点插入最小堆中。如果堆顶元素为空，则跳过。

重复执行步骤 2 直到最小堆为空。

实现的时候，为了保证正确性，需要重载链表的 PartialOrd 和 Ord，以便在插入最小堆时按照链表节点的大小关系进行比较。具体实现过程中，我们可以使用 Rust 的特质（Trait）机制来重载这两个操作。



## 自定义链表


`TreeNodePtr ` 表示返回值可以是一个空指针，或者一个指向堆上 `ListNode` 对象的指针。使用 `Box` 是为了避免在函数返回后出现生命周期问题，直接将 `ListNode` 放在堆上管理。下面是一个简单的示例：

```rust
struct ListNode {
    val: i32,
    next: TreeNodePtr ,
}

fn create_linked_list(values: &Vec<i32>) -> TreeNodePtr  {
    let mut head = None;

    for &val in values.iter().rev() {
        let node = ListNode {
            val,
            next: head.take(),
        };

        head = Some(Box::new(node));
    }

    head
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];
    let head = create_linked_list(&values);

    println!("{:?}", head);
}

```


### 自定义宏

首先，我们需要了解 Rust 的宏基础语法和规则。

Rust 中宏由两部分组成：模式和替换体。模式指定了宏匹配的语法结构，替换体则是匹配到宏之后要生成的代码。模式和替换体之间使用 => 符号分隔。

现在我们要自定义一个宏，让它创建一个链表，最后以 `TreeNodePtr ` 的形式返回链表头指针。

下面是一个简单的示例：

```rust
macro_rules! linked_list {
    // 匹配到空的列表
    () => {
        None
    };
    // 匹配到非空的列表
    ( $( $val:expr ),* ) => {{
        let mut head = None;

        $(
            let node = ListNode {
                val: $val,
                next: head.take(),
            };

            head = Some(Box::new(node));
        )*

        head
    }};
}

```

这个宏定义了两个模式，一个是匹配空列表的模式，一个是匹配非空列表的模式。在匹配到非空列表时，宏会使用一个闭包来创建链表节点并返回链表头指针。

现在我们可以使用 linked_list! 宏来创建链表了：

```rust
let list = linked_list![1, 2, 3, 4];
```

这将会创建一个包含四个节点的链表，并返回链表头指针。

注意：本示例中的 ListNode 结构体需要自行定义。
