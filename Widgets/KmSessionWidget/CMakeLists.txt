include(${KM_CMAKE_DIR}/Widgets.cmake)

add_library(
    KmSessionWidget
)

target_link_libraries(
    KmSessionWidget
    PUBLIC
        Qt5::Core
        Qt5::Widgets
        Qt5::Gui
        KmCommon
        KmApiCommunication
        KmBaseWidget
)

target_sources(
    KmSessionWidget
    PRIVATE
        KmSessionWidget.ui
        KmSessionWidget.cpp
        KmSessionWidget.h
)

target_include_directories(
    KmSessionWidget 
    PUBLIC $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/>
           $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)

install(
    TARGETS KmSessionWidget
    EXPORT KitchenManagerWidgets-config
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR})

install(
    DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/
    DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/KmSessionWidget
    FILES_MATCHING PATTERN "*.h"
)
