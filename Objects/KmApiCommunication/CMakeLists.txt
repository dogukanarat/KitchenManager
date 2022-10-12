include(${KM_CMAKE_DIR}/Objects.cmake)

add_library(
    KmApiCommunication
)

target_link_libraries(
    KmApiCommunication
    PUBLIC
        Qt5::Core
        Qt5::Sql
        KmCommon
)

target_sources(
    KmApiCommunication
    PRIVATE
        KmApiCommunication.cpp
        KmApiCommunication.h
)

target_include_directories(
    KmApiCommunication 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmApiCommunication
    EXPORT KitchenManagerObjects-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmApiCommunication
    FILES_MATCHING PATTERN "*.h"
)
