use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};

type PoolInner<T> = RefCell<VecDeque<Box<T>>>;

/// Maintains a pool of items
#[derive(Debug)]
pub struct Pool<T> {
    items: Rc<PoolInner<T>>,
}

impl<T> Pool<T> {
    /// Creates a new empty pool
    #[inline]
    pub fn new() -> Self {
        Self { items: Rc::new(RefCell::new(VecDeque::new())) }
    }

    /// Borrows an item from the pool. If there is no item available or all items are borrowed,
    /// it returns [`None`].
    #[must_use]
    pub fn borrow(&self) -> Option<PoolRef<T>> {
        self.items.borrow_mut().pop_front()
            .map(|item| PoolRef {
                item: Some(item),
                pool: Rc::downgrade(&self.items)
            })
    }

    /// Borrows an item from the pool. If there is no item available or all items are borrowed,
    /// it uses given closure to create one and returns it.
    #[inline]
    #[must_use]
    pub fn borrow_or_build<F>(&self, build: F) -> PoolRef<T>
    where
        F: FnOnce() -> T,
    {
        self.try_borrow_or_build(|| Result::<T, ()>::Ok(build())).unwrap()
    }


    /// Borrows an item from the pool. If there is no item available or all items are borrowed,
    /// it uses given closure to create one and returns it.
    ///
    /// Can fail if closure fails.
    pub fn try_borrow_or_build<F, E>(&self, build: F) -> Result<PoolRef<T>, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        if let Some(borrow) = self.borrow() {
            Ok(borrow)
        } else {
            Ok(PoolRef {
                item: Some(Box::new(build()?)),
                pool: Rc::downgrade(&self.items)
            })
        }
    }
}

impl<T> Default for Pool<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Reference to a pool's item. Returns the item to the pool when dropped.
#[derive(Debug)]
pub struct PoolRef<T> {
    item: Option<Box<T>>,
    pool: Weak<PoolInner<T>>,
}

impl<T> PoolRef<T> {
    /// Consume the reference to return the item. The item will no longer be returned to the pool.
    ///
    /// Can return [`None`] if the reference has already been leaked.
    #[inline]
    #[must_use]
    pub fn leak(mut self) -> Option<Box<T>> {
        self.item.take()
    }
}

impl<T> AsRef<T> for PoolRef<T> {
    fn as_ref(&self) -> &T {
        self.item.as_ref().unwrap()
    }
}

impl<T> AsMut<T> for PoolRef<T> {
    fn as_mut(&mut self) -> &mut T {
        self.item.as_mut().unwrap()
    }
}

impl<T> Deref for PoolRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.item.as_deref().unwrap()
    }
}

impl<T> DerefMut for PoolRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.item.as_deref_mut().unwrap()
    }
}

impl<T> Drop for PoolRef<T> {
    fn drop(&mut self) {
        if let Some((pool, item)) = self.pool.upgrade().zip(self.item.take()) {
            pool.borrow_mut().push_back(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn borrow_should_return_none_on_empty_pool() {
        let pool: Pool<&str> = Pool::new();
        assert!(pool.borrow().is_none());
    }
    
    #[test]
    fn borrow_or_build_should_use_closure_on_empty_pool() {
        let pool = Pool::new();
        
        assert_eq!(*pool.borrow_or_build(|| "a"), "a");
        assert_eq!(*pool.borrow_or_build(|| "b"), "a"); // <= returns "a", because reference has been dropped on previous line
    }
    
    #[test]
    fn try_borrow_or_build_should_use_closure_on_empty_pool() {
        let pool = Pool::new();

        assert!(pool.try_borrow_or_build(|| Err(())).is_err());
        assert_eq!(pool.try_borrow_or_build(|| Result::<&str, ()>::Ok("a")).as_deref(), Ok(&"a"));
        assert_eq!(pool.try_borrow_or_build(|| Result::<&str, ()>::Ok("b")).as_deref(), Ok(&"a")); // <= returns "a", because reference has been dropped on previous line
        assert_eq!(pool.try_borrow_or_build(|| Err(())).as_deref(), Ok(&"a")); // <= is successful since build was not called, has "a" is available
    }
    
    #[test]
    fn pool_ref_should_return_item_to_pool_when_dropped() {
        let pool = Pool::new();

        {
            let _borrow = pool.borrow_or_build(|| "a");
            assert!(pool.borrow().is_none());
        } // <= drop borrow
        
        assert_eq!(pool.borrow().as_deref(), Some(&"a"));
    }
    
    #[test]
    fn leaked_pool_ref_should_not_return_item_to_pool_when_dropped() {
        let pool = Pool::new();

        {
            let borrow = pool.borrow_or_build(|| "a");
            
            assert_eq!(borrow.leak().as_deref(), Some(&"a"));
            assert!(pool.borrow().is_none());
        } // <= drop borrow

        assert!(pool.borrow().is_none());
    }
}