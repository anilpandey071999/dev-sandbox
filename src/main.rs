// use std::io::{self, Write, stdin, stdout};

use crate::{
    documents::{Document, Documents},
    search_engine::SearchEngine,
};

pub mod documents;
pub mod search_engine;
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

// fn get_user_input() {
//     let mut documents = Documents::new();
//     let mut hash = SearchEngine::new();
//     loop {
//         print!(
//             "> Choose the option \n> 1 for adding document \n> 2 for searching \n> 3 for seen all the documents \n> 4 for exiting \n>"
//         );
//         stdout().flush().unwrap();

//         let mut choose = String::new();
//         stdin().read_line(&mut choose).unwrap();
//         // if choose.contains("exits") {
//         //     break;
//         // }
//         let mut title = String::new();
//         let mut content = String::new();
//         let choose = choose.trim().parse::<i32>().unwrap();
//         match choose {
//             1 => adding_document(&mut documents, &mut hash, &mut title, &mut content).unwrap(),
//             2 => searching_doc(&mut documents, &mut hash),
//             // 3 =>
//             _ => println!("invalied choose"),
//         }
//     }
// }

// fn adding_document<'a>(
//     documents: &'a mut Documents<'a>,
//     hash: &'a mut SearchEngine<'a>,
//     title: &'a mut String,
//     content: &'a mut String,
// ) -> Result<(), String> {
//     println!("> Enter the title (or 'exit' to quit): ");
//     stdout().flush().unwrap();

//     stdin().read_line(title).unwrap();
//     let title = title.trim().to_string();

//     if title.eq_ignore_ascii_case("exit") {
//         println!("> Exiting...");
//         return Err("Exiting...".to_string());
//     }

//     println!("Enter document content: ");
//     stdout().flush().unwrap();

//     stdin().read_line(content).unwrap();
//     let content = content.trim().to_string();

//     documents.insert_documents(title.as_str(), content.as_str(), hash);

//     Ok(())
// }

// fn searching_doc(documents: &mut Documents, hash: &mut SearchEngine) {
//     println!("yet to be implemented...");
// }

// fn viewing_docs() {
//     println!("{:?}", )
// }
