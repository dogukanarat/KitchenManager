include(${KM_CMAKE_DIR}/Widgets.cmake)

add_library(
    KmOrderViewWidget
)

target_link_libraries(
    KmOrderViewWidget
    PUBLIC
        Qt5::Core
        Qt5::Widgets
        KmCommon
        KmBaseWidget
)

target_sources(
    KmOrderViewWidget
    PRIVATE
        KmOrderViewWidget.cpp
        KmOrderViewWidget.h
        KmOrderViewWidget.ui
)

target_include_directories(
    KmOrderViewWidget 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmOrderViewWidget
    EXPORT KitchenManagerWidgets-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmOrderViewWidget
    FILES_MATCHING PATTERN "*.h"
)
