// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `org.jspecify.annotations.Nullable` -- compile-time-only
// nullability annotation, no runtime behaviour.
//
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package org.jspecify.annotations;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Retention(RetentionPolicy.CLASS)
@Target({ElementType.TYPE_USE, ElementType.TYPE_PARAMETER})
public @interface Nullable {
}