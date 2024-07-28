// To achieve faster results in priority queues, a binary heap can be used to sort the priority queue.
// A binary heap is a complete binary tree and allows queuin and dequeuing in O(log(n)) time, making it 
// ideal for efficient scheduling systems.

// There are two forms of binary heaps: the min heap and the max heap.

// ************ Binary Heap Abstract Data Type
// A min-heap can be used as a priority queue
// new() ---- creates a new binary heap with no parameters and returns nothing.
// push(k) ---- adds a new item with parameter value k to the heap and rtns nothing.
// pop() ---- returns and removes the minimum item from the heap and modifies the heap.
// min() ---- returns the minimum items in the heap as a numerical value.
// size() ---- returns the number of items in the heap as a numerical value.
// is_empty() ---- returns a Boolean value indicating whether the heap is empty or not.
// build(arr) ---- constructs a new heap from an array or vector of data, with arr as the parameter.


// For a binary heap stored in a linear data structure, keeping it balanced is key to achieving logarithmic
// performance.
// A balanced binary heap has approximately equal numbers of nodes in its left and right subtrees,
// and it strives to fill each node's left and right child nodes, with at most one node havng a non-full
// set of children in the worst case.


// As the parent and child nodes are stored in a linear ds. If a node is at index p, then
// its left child node is at 2p, and its right child node is at 2p + 1.
// The parent node of a child node at index c is at index c/2.

// calculate parent node index
macro_rules! parent {
    ($child: ident) => {
        $child >> 1
    };
}

// calculate left child node index
macro_rules! left_child {
    ($parent: ident) => {
        $parent << 1
    };
}

// calculate right child node index
macro_rules! right_child {
    ($parent: ident) => {
        ($parent << 1) + 1
    };
}


// The binary heap is defined as a data structure that includes a field representing the size of the heap /
// The size field does not include the first data item 0, which is considered a placeholder. /
// The data saved in the heap is assumed to be i32 /


// Implement Debug and Clone Trait
#[derive(Debug, Clone)]
struct BinaryHeap{
    size: usize, // data count
    data: Vec<i32>, // data storage
}

// Implement the binary heap
impl BinaryHeap {
    fn new() -> Self {
        BinaryHeap {
            size: 0,
            data: vec![0],
        }
    }
    
    fn size(&self) -> usize {
        self.size
    }
    
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    
    fn min(&self) -> Option<i32> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[1])
            // Some(self.data[1].clone())
            // clone used for generic types
        }
    }

    // When adding data to the heap, adding it to the end of the heap will disrupt the balance, so the data
    // needs to be moved up to maintain balance.
    
    // Add a data to the end and adjust the heap
    fn push(&mut self, val: i32) {
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }
    
    // smaller data is moved up
    // c(child, current), p(parent)
    fn move_up(&mut self, mut c: usize) {
        let mut p = parent!(c);
        while p > 0 {
            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
                c = p;
                p = parent!(c);
            } else {
                break;
            }
        }
    }
    
    // pop out the top value
    fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            None
        } else if 1 == self.size {
            self.size -= 1;
            self.data.pop()
        }else { 
            // swap data and then adjust the heap
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            
            val
        }
    }
    
    // larger data is moved down
    /*fn move_down(&mut self, mut c: usize) {
        loop {
            let l = left_child!(c);
            let r = right_child!(c);
            let mut min = c;
            
            if l <= self.size && self.data[l] < self.data[min] {
                min = l;
            }
            
            if r <= self.size && self.data[r] < self.data[min] {
                min = r;
            }
            
            if min != c {
                self.data.swap(c, min);
                c = min;
            } else {
                break;
            }
        }
    }*/
    
    // move bigger data down
    fn move_down(&mut self, mut c: usize) {
        loop {
            let lc = left_child!(c);
            if lc > self.size {break;}
            
            // the index of minimum child node of current node
            let mc = self.min_child(c);
            if self.data[c] > self.data[mc] { 
                self.data.swap(c, mc);
            }
            // the minimum child node becomes current node
            c = mc;
        }
    }
    
    
    // Calculate the index of the minimum child node
    fn min_child(&self, c: usize) -> usize {
        let(rc, lc) = (left_child!(c), right_child!(c));
        
        if rc > self.size {
            // right child node is out of range
            // left child node is the minimum child node
            lc
        } else if self.data[lc] < self.data[rc] {
            // left child node is smaller than right child node
            lc
        } else {
            // right child node is smaller than left child node
            rc
        }
    }
    
    // build a new heap
    fn build_new(&mut self, arr: &[i32]) {
        // clear the original data
        for _i in 0..self.size {
            self.data.pop();
        }
        
        // add new data
        self.size = arr.len();
        for x in arr {
            self.data.push(*x);
        }
        
        // adjust the heap to make it a min-heap
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            self.move_down(p);
            p -= 1;
        }
        
    }
    
}














