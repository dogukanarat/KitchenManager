add_subdirectory(KmApiCommunication)

install(
    EXPORT KitchenManagerObjects-config
    NAMESPACE KitchenManagerObjects::
    DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake
)
