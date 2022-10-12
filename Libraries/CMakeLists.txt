add_subdirectory(KmCommon)

install(
    EXPORT KitchenManagerLibraries-config
    NAMESPACE KitchenManagerLibraries::
    DESTINATION ${CMAKE_INSTALL_PREFIX}/cmake
)
