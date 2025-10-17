use std::io::{self, Write, stdin, stdout};

use std::sync::{Arc, Mutex};
use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use crate::{
    documents::{Document, Documents},
    search_engine::SearchEngine,
};

pub mod documents;
pub mod search_engine;

static GLOBLE_DATA_BASE: LazyLock<Mutex<HashMap<u64, (String, String)>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn main() {
    println!("Hello, world!");
    let mut documents = Documents::new();
    let records = vec![
        Document {
            title: "Introduction to Rust",
            contents: "Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without garbage collection.",
        },
        Document {
            title: "Understanding Ownership",
            contents: "Ownership is Rust's most unique feature. Each value has a variable that's called its owner. There can only be one owner at a time.",
        },
        Document {
            title: "Vector Data Structures",
            contents: "Vectors are resizable arrays stored on the heap. They can grow or shrink in size and are one of the most commonly used collection types.",
        },
        Document {
            title: "Hash Maps and Caching",
            contents: "Hash maps store key-value pairs and provide O(1) average lookup time. They're essential for implementing caching mechanisms efficiently.",
        },
        Document {
            title: "Search Algorithms",
            contents: "Binary search, linear search, and hash-based search are fundamental algorithms. Each has different time complexity and use cases.",
        },
        Document {
            title: "Database Indexing",
            contents: "Indexes improve query performance by creating data structures that allow fast retrieval. Common types include B-tree and hash indexes.",
        },
        Document {
            title: "Memory Management",
            contents: "Efficient memory management is crucial for performance. Techniques include pooling, caching, and careful allocation strategies.",
        },
        Document {
            title: "String Processing",
            contents: "String manipulation involves operations like concatenation, splitting, searching, and pattern matching. Performance varies by implementation.",
        },
        Document {
            title: "Concurrency Patterns",
            contents: "Concurrent programming requires careful synchronization. Patterns include mutexes, channels, and lock-free data structures.",
        },
        Document {
            title: "Error Handling",
            contents: "Rust uses Result and Option types for error handling. This approach makes errors explicit and forces developers to handle them.",
        },
        Document {
            title: "Testing Best Practices",
            contents: "Good tests are isolated, repeatable, and fast. Unit tests verify individual components while integration tests check system behavior.",
        },
        Document {
            title: "Full-Text Search",
            contents: "Full-text search enables finding documents containing specific words or phrases. Techniques include inverted indexes and tokenization.",
        },
        Document {
            title: "Query Optimization",
            contents: "Query optimization improves search performance through caching, indexing, and query rewriting. Cache hit rates significantly impact speed.",
        },
        Document {
            title: "Data Serialization",
            contents: "Serialization converts data structures into formats for storage or transmission. Popular formats include JSON, MessagePack, and Protocol Buffers.",
        },
        Document {
            title: "Benchmark Methodology",
            contents: "Proper benchmarking requires controlling variables, warming up caches, and running multiple iterations. Statistical analysis helps identify real improvements.",
        },
        Document {
            title: "API Design Principles",
            contents: "Good APIs are intuitive, consistent, and hard to misuse. Clear documentation and examples help developers use them effectively.",
        },
        Document {
            title: "Performance Profiling",
            contents: "Profiling identifies bottlenecks in code. Tools measure CPU usage, memory allocation, and cache misses to guide optimization efforts.",
        },
        Document {
            title: "Document Ranking",
            contents: "Search results need ranking to show the most relevant documents first. Algorithms like TF-IDF and BM25 calculate relevance scores.",
        },
        Document {
            title: "Cache Invalidation",
            contents: "Cache invalidation is one of the hardest problems in computer science. Strategies include time-based expiration and event-driven invalidation.",
        },
        Document {
            title: "Distributed Systems",
            contents: "Distributed systems span multiple machines for scalability and reliability. Challenges include consistency, partition tolerance, and latency.",
        },
    ];

    // let records = vec![
    //     Document {
    //         title: "Introduction to Rust",
    //         contents: "Rust is a systems programming language focused on safety, speed, and concurrency. It achieves memory safety without garbage collection.",
    //     },
    //     Document {
    //         title: "Understanding Ownership",
    //         contents: "Ownership is Rust's most unique feature. Each value has a variable that's called its owner. There can only be one owner at a time.",
    //     },
    //     Document {
    //         title: "Vector Data Structures Rust",
    //         contents: "Vectors are resizable arrays stored on the heap. They can grow or shrink in size and are one of the most commonly used collection types.",
    //     },
    // ];
    println!("len of recodes :{}", records.len());
    let mut search_engine = SearchEngine::new();
    for (_i, doc) in records.iter().enumerate() {
        documents.insert_documents(doc.title, doc.contents, &mut search_engine);
        // if i == 10 {
        //     println!("{:?}", search_engine.search);
        // }
    }

    // if let Err(err) = documents.delete_documents(19) {
    //     println!("{err}");
    // }

    // println!("{}", documents.docs.l
    // en());

    let serach_key = "and";

    println!("{:?}", search_engine.search.get(serach_key));

    match documents.search_key(serach_key, &search_engine) {
        Ok(val) => val
            .iter()
            .for_each(|val| println!("title: {}\n contect: {}", val.title, val.contents)),
        Err(err) => eprintln!("{}", err),
    }
    // get_user_input()
}

fn get_user_input() {
    let mut documents = Documents::new();
    let mut hash = SearchEngine::new();
    loop {
        print!(
            "> Choose the option \n> 1 for adding document \n> 2 for searching \n> 3 for seen all the documents \n> 4 for exiting \n>"
        );
        stdout().flush().unwrap();

        let mut choose = String::new();
        stdin().read_line(&mut choose).unwrap();
        // if choose.contains("exits") {
        //     break;
        // }
        let choose = choose.trim().parse::<i32>().unwrap();
        match choose {
            1 => adding_document(&mut documents, &mut hash).unwrap(),
            // 2 => searching_doc(&mut documents, &mut hash),
            // 3 =>
            _ => println!("invalied choose"),
        }
    }
}

fn adding_document<'a>(
    documents: &'a mut Documents<'a>,
    hash: &'a mut SearchEngine<'a>,
) -> Result<(), String> {
    println!("> Enter the title (or 'exit' to quit): ");
    stdout().flush().unwrap();

    let mut title = String::new();
    let mut content = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim().to_string();

    if title.eq_ignore_ascii_case("exit") {
        println!("> Exiting...");
        return Err("Exiting...".to_string());
    }

    println!("Enter document content: ");
    stdout().flush().unwrap();

    stdin().read_line(&mut content).unwrap();
    let content = content.trim().to_string();

    let _ = GLOBLE_DATA_BASE.lock().unwrap().insert(1, (title, content));
    // let get_value = GLOBLE_DATA_BASE.read().unwrap().get(&1).unwrap();
    // let reader = GLOBLE_DATA_BASE.read().unwrap().clone();
    // let gard = GLOBLE_DATA_BASE;
    // let (tit, content) = *gard.get(&1).unwrap();
    if let Some((title, content)) = GLOBLE_DATA_BASE.lock().unwrap().get(&1) {
        // let a = Arc::new(tit)
        documents.insert_documents(title, content, hash);
    }
    // documents.insert_documents(
    //     &*GLOBLE_DATA_BASE.lock().unwrap().get(&1).unwrap().0,
    //     &*GLOBLE_DATA_BASE.lock().unwrap().get(&1).unwrap().0,
    //     hash,
    // );
    // }

    Ok(())
}

fn searching_doc(documents: &mut Documents, hash: &mut SearchEngine) {
    println!("yet to be implemented...");
}

// fn viewing_docs() {
//     println!("{:?}",)
// }


// pub struct Node {
//     pub data: i32,
//     pub next: Option<Box<Node>>,
// }
// impl Node {
//     pub fn new(data: i32) -> Self {
//         Self { data, next: None }
//     }
// }

// pub struct LinkList {
//     pub head: Option<Box<Node>>,
// }
// impl LinkList {
//     pub fn new(data: i32) -> Self {
//         Self {
//             head: Some(Box::new(Node::new(data))),
//         }
//     }

//     pub fn delete_at_end(&mut self) {
//         let mut current = &mut self.head;
//         while let Some(node) = current {
//             if !node.as_ref().next.is_none() && node.next.as_ref().unwrap().next.is_none() {
//                 let deleted_node = node.next.take();
//                 node.next = None;
//                 println!("deleted node {}", deleted_node.unwrap().data);
//                 return;
//             }
//             current = &mut node.next;
//         }
//     }

//     pub fn push_at_end(&mut self, data: i32) {
//         let new_node = Box::new(Node::new(data));
//         let mut current = &mut self.head;
//         while let Some(node) = current {
//             if node.next.is_none() {
//                 node.next = Some(new_node);
//                 return;
//             }
//             current = &mut node.next;
//         }
//     }

//     pub fn delete_at_frount(&mut self) {
//         let deleted_node = self.head.take();
//         println!("deleted node {}", deleted_node.as_ref().unwrap().data);
//         self.head = deleted_node.unwrap().next;
//     }

//     pub fn push_at_frount(&mut self, data: i32) {
//         let mut new_head = Box::new(Node::new(data));
//         let head = self.head.take();
//         new_head.next = head;
//         self.head = Some(new_head);
//     }

//     pub fn push_at_pos(&mut self, data: i32, pos: u32) {
//         let mut current = &mut self.head;
//         let mut node_tracker = 1;
//         while let Some(node) = current {
//             if pos == node_tracker {
//                 let take_data = node.next.take();
//                 let new_node = Box::new(Node {
//                     data,
//                     next: take_data,
//                 });
//                 node.next = Some(new_node);
//                 return;
//             }
//             node_tracker += 1;
//             current = &mut node.next;
//         }
//     }
// }

// impl std::fmt::Display for LinkList {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut current = &self.head;
//         while let Some(node) = current {
//             write!(f, r#"Node Data {} -> "#, node.data).unwrap();
//             current = &node.next;
//         }
//         Ok(())
//     }
// }

// fn main() {
//     let mut link_list = LinkList::new(0);

//     for i in 0..10 {
//         link_list.push_at_end(i);
//     }

//     for i in 11..20 {
//         link_list.push_at_frount(i);
//     }

//     link_list.push_at_pos(100, 3);
//     // link_list.push_at_end(10);
//     println!("{}", link_list);

//     link_list.delete_at_end();

//     link_list.delete_at_frount();
//     println!("{}", link_list);
// }
