add_subdirectory(KitchenManagerViewer)

install(
    EXPORT KitchenManagerViewer-config
    NAMESPACE KitchenManager::
    DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake
)
