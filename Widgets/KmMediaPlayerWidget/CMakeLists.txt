include(${KM_CMAKE_DIR}/Widgets.cmake)

add_library(
    KmMediaPlayerWidget
)

target_link_libraries(
    KmMediaPlayerWidget
    PUBLIC
        Qt5::Core
        Qt5::Widgets
        Qt5::Multimedia
        KmCommon
        KmBaseWidget
)

target_sources(
    KmMediaPlayerWidget
    PRIVATE
        KmMediaPlayerWidget.cpp
        KmMediaPlayerWidget.h
        KmMediaPlayerWidget.ui
)

target_include_directories(
    KmMediaPlayerWidget 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmMediaPlayerWidget
    EXPORT KitchenManagerWidgets-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmMediaPlayerWidget
    FILES_MATCHING PATTERN "*.h"
)
