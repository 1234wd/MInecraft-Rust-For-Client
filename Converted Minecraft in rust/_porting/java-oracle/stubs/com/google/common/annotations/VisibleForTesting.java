// ============================================================================
// ORACLE STUB -- NOT GAME CODE, NOT part of the Rust mirror.
//
// Stands in for `com.google.common.annotations.VisibleForTesting` so that the
// ORIGINAL, UNMODIFIED sources in minecraft-decompiled/ can be compiled by
// _porting/java-oracle.
//
// Why: compile-time-only annotation, has no runtime behaviour at all.
// Rule: a stub may only supply types/methods that the ported Java classes merely
// *touch*. It must never change the behaviour of a method under parity test.
// See _porting/DESIGN_DECISIONS.md (#stubs).
// ============================================================================
package com.google.common.annotations;

import java.lang.annotation.Documented;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

@Documented
@Retention(RetentionPolicy.CLASS)
@Target({ElementType.CONSTRUCTOR, ElementType.FIELD, ElementType.METHOD, ElementType.TYPE})
public @interface VisibleForTesting {
}