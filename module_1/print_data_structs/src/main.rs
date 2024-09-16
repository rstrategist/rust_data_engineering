use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");

    // Examples
    println!("\n\tExamples:");

    // Sequences
    println!("\n\tSequences:");

    // Vec
    println!("\t\tVec:");
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("\t\tInserted elements in Vec: {:?}", vec);
    vec.pop();
    println!("\t\tAfter popping an element from Vec: {:?}", vec);

    // VecDeque
    println!("\t\tVecDeque:");
    let mut deque = VecDeque::new();
    deque.push_front(1);
    deque.push_back(2);
    deque.push_back(3);
    println!("\t\tInserted elements in VecDeque: {:?}", deque);
    deque.pop_front();
    println!(
        "\t\tAfter popping an element from the front of VecDeque: {:?}",
        deque
    );
    deque.pop_back();
    println!(
        "\t\tAfter popping an element from the back of VecDeque: {:?}",
        deque
    );

    // LinkedList
    println!("\t\tLinkedList:");
    let mut linked_list = LinkedList::new();
    linked_list.push_back(4);
    linked_list.push_front(3);
    linked_list.push_back(5);
    println!("\t\tInserted elements in LinkedList: {:?}", linked_list);
    linked_list.pop_front();
    println!(
        "\t\tAfter popping an element from front of LinkedList: {:?}",
        linked_list
    );
    linked_list.pop_back();
    println!(
        "\t\tAfter popping an element from back of LinkedList: {:?}",
        linked_list
    );

    // Maps
    println!("\n\tMaps:");

    // HashMap
    println!("\t\tHashMap:");
    let mut hashmap = HashMap::new();
    hashmap.insert("key1", "value1");
    hashmap.insert("key2", "value2");
    println!("\t\tInserted key-value pairs in HashMap: {:?}", hashmap);
    hashmap.remove("key1");
    println!(
        "\t\tAfter removing key1 with hashmap.remove(\"key1\") from HashMap: {:?}",
        hashmap
    );

    // BTreeMap
    println!("\t\tBTreeMap:");
    let mut btree_map = BTreeMap::new();
    btree_map.insert(1, "one");
    btree_map.insert(2, "two");
    println!("\t\tInserted key-value pairs in BTreeMap: {:?}", btree_map);
    btree_map.remove(&1);
    println!("\t\tAfter removing a key from BTreeMap: {:?}", btree_map);

    // Sets
    println!("\n\tSets:");

    // HashSet
    println!("\t\tHashSet:");
    let mut hash_set = HashSet::new();
    hash_set.insert(5);
    hash_set.insert(6);
    println!("\t\tInserted elements in HashSet: {:?}", hash_set);
    hash_set.remove(&5);
    println!("\t\tAfter removing an element from HashSet: {:?}", hash_set);

    // BTreeSet
    println!("\t\tBTreeSet:");
    let mut btree_set = BTreeSet::new();
    btree_set.insert(7);
    btree_set.insert(8);
    println!("\t\tInserted elements in BTreeSet: {:?}", btree_set);
    btree_set.remove(&7);
    println!(
        "\t\tAfter removing an element from BTreeSet: {:?}",
        btree_set
    );

    // Misc
    println!("\n\tMisc:");

    // BinaryHeap
    println!("\t\tBinaryHeap:");
    let mut binary_heap = BinaryHeap::new();
    binary_heap.push(10);
    binary_heap.push(5);
    println!("\t\tInserted elements in BinaryHeap: {:?}", binary_heap);
    binary_heap.pop(); //pops first element as opposed to vec which pops the last element
    println!(
        "\t\tAfter popping an element from BinaryHeap: {:?}",
        binary_heap
    );
}
