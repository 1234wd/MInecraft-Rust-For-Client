//! Port of: net/minecraft/client/multiplayer/CacheSlot.java
//! Java class(es): net.minecraft.client.multiplayer.CacheSlot, net.minecraft.client.multiplayer.Cleaner
//! Status: PORTED
//!
//! A cache slot that holds the result of applying a `Function<C, D>` to a context `C`, keyed by
//! reference equality on the context. `compute(context)` returns the cached value when the same
//! context (by identity) is passed again, otherwise recomputes and re-caches. `clear()` drops
//! the cached value and context.
//!
//! External contracts:
//! - `java.util.function.Function<C, D>` -> `javacompat::java_concurrent::Function` (modeled).
//! - `org.jspecify.annotations.Nullable` -> marker annotation, no runtime effect.
//!
//! Reference equality on the context is modeled with `Rc::ptr_eq`, matching Java's `==` on
//! object references. The context `C` must implement `Cleaner<C>` so that `compute` can call
//! `context.registerForCleaning(this)`, exactly as in Java.

use std::rc::Rc;

use crate::javacompat::java_concurrent::Function;

/// Port of `CacheSlot.Cleaner<C>`, a `@FunctionalInterface` with `registerForCleaning`.
pub trait Cleaner<C> {
    /// Port of `void registerForCleaning(CacheSlot<C, ?> slot)`.
    ///
    /// The wildcard `?` on `D` means any `CacheSlot<C, D>` can be registered; the cleaner
    /// receives an immutable reference and can read from the slot (e.g. to invalidate later).
    fn register_for_cleaning<D>(&self, slot: &CacheSlot<C, D>);
}

/// Port of `net.minecraft.client.multiplayer.CacheSlot<C, D>`.
pub struct CacheSlot<C, D> {
    operation: Box<dyn Function<C, D>>,
    context: Option<Rc<C>>,
    value: Option<Rc<D>>,
}

impl<C: Cleaner<C> + Clone, D> CacheSlot<C, D> {
    /// Port of `CacheSlot(Function<C, D> operation)`.
    pub fn new(operation: Box<dyn Function<C, D>>) -> Self {
        CacheSlot {
            operation,
            context: None,
            value: None,
        }
    }

    /// Port of `D compute(C context)`.
    ///
    /// Returns the cached value (by shared reference) when `context` is the same object as the
    /// previously cached one; otherwise applies `operation`, stores the result, registers the
    /// slot with the context for cleaning, and returns the new value.
    pub fn compute(&mut self, context: Rc<C>) -> Rc<D> {
        if let (Some(ref ctx), Some(ref v)) = (&self.context, &self.value) {
            if Rc::ptr_eq(ctx, &context) {
                return Rc::clone(v);
            }
        }
        let c = context.as_ref().clone();
        let new_value = self.operation.apply(c);
        let new_value_rc = Rc::new(new_value);
        self.context = Some(Rc::clone(&context));
        self.value = Some(Rc::clone(&new_value_rc));
        context.register_for_cleaning(self);
        new_value_rc
    }

    /// Port of `void clear()`.
    pub fn clear(&mut self) {
        self.value = None;
        self.context = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CountingFunc;

    impl Function<CleanContext, String> for CountingFunc {
        fn apply(&self, t: CleanContext) -> String {
            format!("{}:{}", 1, t.0)
        }
    }

    #[derive(Clone)]
    struct CleanContext(i32);

    impl Cleaner<CleanContext> for CleanContext {
        fn register_for_cleaning<D>(&self, _slot: &CacheSlot<CleanContext, D>) {
            // no-op for the test
        }
    }

    #[test]
    fn compute_caches_by_context_identity() {
        let op = Box::new(CountingFunc);
        let mut slot: CacheSlot<CleanContext, String> = CacheSlot::new(op);

        let ctx1 = Rc::new(CleanContext(7));
        let r1 = slot.compute(Rc::clone(&ctx1));
        let r2 = slot.compute(Rc::clone(&ctx1));
        assert!(Rc::ptr_eq(&r1, &r2), "same context must return cached value");

        let ctx2 = Rc::new(CleanContext(8));
        let r3 = slot.compute(Rc::clone(&ctx2));
        assert!(!Rc::ptr_eq(&r1, &r3), "different context must recompute");
    }

    #[test]
    fn clear_invalidates_cache() {
        let op = Box::new(CountingFunc);
        let mut slot: CacheSlot<CleanContext, String> = CacheSlot::new(op);

        let ctx = Rc::new(CleanContext(3));
        let r1 = slot.compute(Rc::clone(&ctx));
        slot.clear();
        let r2 = slot.compute(Rc::clone(&ctx));
        assert!(!Rc::ptr_eq(&r1, &r2), "after clear, compute must recompute");
    }
}
