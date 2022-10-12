add_subdirectory(KmBaseWidget)
add_subdirectory(KmMediaPlayerWidget)
add_subdirectory(KmOrderViewWidget)
add_subdirectory(KmSessionWidget)

install(
    EXPORT KitchenManagerWidgets-config
    NAMESPACE KitchenManagerWidgets::
    DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake
)
