{
  pkgs,
  buildInputs,
  ...
}: {
  # Wayland configuration
  WAYLAND_DISPLAY = "wayland-1";
  XDG_SESSION_TYPE = "wayland";
  XDG_RUNTIME_DIR = "/run/user/1000";
  WINIT_UNIX_BACKEND = "wayland";
  WAYLAND_DEBUG = "0";

  # Library paths
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
  OPENSSL_DIR = "${pkgs.openssl}";
  OPENSSL_NO_VENDOR = "1";
  OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
  OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.wayland.dev}/lib/pkgconfig";

  # Font configuration
  XDG_DATA_DIRS = "${pkgs.dejavu_fonts}/share:${pkgs.noto-fonts}/share:${pkgs.liberation_ttf}/share";
}
