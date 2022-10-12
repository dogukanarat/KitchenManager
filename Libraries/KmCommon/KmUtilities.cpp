#include "KmCommon/KmUtilities.h"
#include <cstring>

using namespace KitchenManager;

VoidPointer Utilities::memSet(VoidPointer destination, Int32 value, Size length)
{
    return memset(destination, value, length);
}

VoidPointer Utilities::memCopy(VoidPointer destination, ConstVoidPointer source, Size length)
{
    return memcpy(destination, source, length);
}
