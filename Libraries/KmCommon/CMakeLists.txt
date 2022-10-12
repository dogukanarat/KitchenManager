include(${KM_CMAKE_DIR}/Libraries.cmake)

add_library(
    KmCommon
)

target_link_libraries(
    KmCommon
    PRIVATE
        Qt5::Core
)

target_sources(
    KmCommon
    PRIVATE
        KmCommon.h
        KmUtilities.cpp
        KmUtilities.h
)

target_include_directories(
    KmCommon 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmCommon
    EXPORT KitchenManagerLibraries-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmCommon
    FILES_MATCHING PATTERN "*.h"
)
