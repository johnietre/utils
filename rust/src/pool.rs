pub type NewFn<T> = Box<dyn Fn() -> Option<T> + Send + Sync + 'static>;

pub struct SyncPool<T: Send + Sync> {
    new_fn: NewFn<T>,
    pool: ANodeAPtr<T>,
}

impl<T: Send + Sync> SyncPool {
    pub fn new(new_fn: Option<NewFn>) -> Self {}

    pub fn get(&self) -> Option<T> {}

    pub fn put(&self, value: T) {}

    fn push_front(&self, value: T) {
        let node_ptr = AtomicNode::new_ptr(value);
        unsafe {
            (*node_ptr).next.store(
                self.pool.swap(node_ptr, Ordering::Acquire),
                Ordering::Release,
            );
        }
    }

    fn pop_front(&self) -> Option<T> {
        let node_ptr = loop {
            let ptr = self.pool.load(Ordering::Acquire);
            if ptr.is_null() {
                break ptr;
            }
            unsafe {
                // TODO: Use strong?
                if !(*ptr)
                    .removed
                    .compare_exchange_weak(true, Ordering::Acquire, Ordering::Release)
                {
                    break ptr;
                }
            }
            spin_loop();
        };
        if node_ptr.is_null() {
            return None;
        }
        unsafe {
            // TODO
        }
    }
}

#[inline(always)]
const fn pending<T>() -> *mut T {
    usize::MAX as _
}

type ANodeAPtr<T> = AtomicPtr<AtomicNode<T>>;

struct AtomicNode<T> {
    value: T,
    next: ANodeAPtr<T>,
    removed: AtomicBool,
}

impl<T> AtomicNode<T> {
    fn new_ptr(value: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            value,
            next: AtomicPtr::new(0 as _),
            removed: AtomicBool::new(false),
        }))
    }
}
