#ifndef INCLUDED_OG_COMMON_H
#define INCLUDED_OG_COMMON_H

#include <cstdint>

#ifndef NULL
#define NULL 0
#endif

#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#ifndef OG_EXPECT
#include <assert.h>
#define OG_EXPECT(A) assert(A);
#endif

#ifndef KM_NO_IMPLEMENTATION
#define KM_NO_IMPLEMENTATION
#endif

#ifndef KM_UNUSED_PARAMETER
#define KM_UNUSED_PARAMETER(A) (void)(A);
#endif

namespace KitchenManager
{
    typedef uint8_t UInt8;
    typedef uint16_t UInt16;
    typedef uint32_t UInt32;
    typedef uint64_t UInt64;
    typedef int8_t Int8;
    typedef int16_t Int16;
    typedef int32_t Int32;
    typedef int64_t Int64;
    typedef float Real32;
    typedef double Real64;
    typedef UInt8 Bool;
    typedef UInt32 Size;

    typedef void Void;
    typedef Void *VoidPointer;
    typedef const Void *ConstVoidPointer;
    typedef UInt8 Byte;
    typedef Byte *BytePointer;
    typedef char Char;
    typedef Char *CharPointer;
}

typedef KitchenManager::UInt8        KUInt8;
typedef KitchenManager::UInt16       KUInt16;
typedef KitchenManager::UInt32       KUInt32;
typedef KitchenManager::UInt64       KUInt64;
typedef KitchenManager::Int8         KInt8;
typedef KitchenManager::Int16        KInt16;
typedef KitchenManager::Int32        KInt32;
typedef KitchenManager::Int64        KInt64;
typedef KitchenManager::Real32       KReal32;
typedef KitchenManager::Real64       KReal64;
typedef KUInt8                       KBool;
typedef KUInt32                      KSize;

#endif // INCLUDED_OG_COMMON_H
