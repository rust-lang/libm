/** This is meant to override Musl's src/include/features.h
 *
 * We 
 * We use a separate file here to redefine some attributes that don't work on
 * all platforms that we would like to build on
 */

#ifndef FEATURES_H
#define FEATURES_H

/* replace `#include "../../include/features.h"` since we can't use the
 * relative path. The C macros need double indirection to get a usable
 * string. */
#define _stringify_inner(s) #s
#define _stringify(s) _stringify_inner(s)
#include _stringify(ROOT_INCLUDE_FEATURES)

#if defined(__APPLE__)
/* Can't define `weak` because that word gets used in `weak_alias` */
#define hidden __attribute__((__visibility__("hidden")))

/* We _should_ be able to define this as:
 *     _Pragma(_stringify(weak musl_ ## new = musl_ ## old))
 * However, weak symbols aren't handled correctly [1]. So we do something
 * extremely hacky and redefine a function that takes 
 *
 * [1]: https://github.com/llvm/llvm-project/issues/111321
 */
#define weak_alias(old, new)


#ifdef __x86_64__
#define _alias_asm_expr(fn_name) __asm__("call " #fn_name "\nret");
#else
#define _alias_asm_expr(fn_name) __asm__("b " #fn_name "\nret");
#endif

#define naked_alias(old, new) \
	__attribute__((naked)) \
	void musl_ ## new() { \
		_alias_asm_expr(_musl_ ## old) \
	}

#else
#define weak __attribute__((__weak__))
#define hidden __attribute__((__visibility__("hidden")))
#define weak_alias(old, new) \
	extern __typeof(old) musl_ ## new \
	__attribute__((__weak__, __alias__(_stringify(musl_ ## old))))

#endif /* defined(__APPLE__) */

#endif
