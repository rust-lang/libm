/* These targets do not have support for weak symbols */
#if defined(__APPLE__) || defined(__MINGW32__)
#include "features.h"

naked_alias(__lgamma_r, lgamma_r)
naked_alias(__lgammaf_r, lgammaf_r)
naked_alias(__lgammal_r, lgammal_r)
naked_alias(__signgam, signgam)
naked_alias(exp10, pow10)
naked_alias(exp10f, pow10f)
naked_alias(exp10l, pow10l)
naked_alias(remainder, drem)
naked_alias(remainderf, dremf)

#endif
