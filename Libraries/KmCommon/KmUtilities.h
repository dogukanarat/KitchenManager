#ifndef INCLUDED_OG_UTILITIES_H
#define INCLUDED_OG_UTILITIES_H

#include "KmCommon/KmCommon.h"

namespace KitchenManager
{
    class Utilities
    {
    public:
        static VoidPointer memSet(VoidPointer destination, Int32 value, Size length);
        static VoidPointer memCopy(VoidPointer destination, ConstVoidPointer source, Size length);
    };
}

#endif // INCLUDED_OG_UTILITIES_H
