include(${KM_CMAKE_DIR}/Widgets.cmake)

add_library(
    KmBaseWidget
)

target_link_libraries(
    KmBaseWidget
    PUBLIC
        Qt5::Core
        Qt5::Widgets
        KmCommon
)

target_sources(
    KmBaseWidget
    PRIVATE
        KmBaseWidget.cpp
        KmBaseWidget.h
)

target_include_directories(
    KmBaseWidget 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmBaseWidget
    EXPORT KitchenManagerWidgets-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmBaseWidget
    FILES_MATCHING PATTERN "*.h"
)
