{
  pkgs,
  toolchain,
  ...
}:
with pkgs; [
  # Essential build dependencies
  pkg-config
  toolchain

  # Font rendering
  fontconfig
  freetype
  freetype.dev
  pango

  # Fonts
  dejavu_fonts
  noto-fonts
  noto-fonts-emoji
  liberation_ttf

  # Security and encryption
  gnupg
  openssl.out
  openssl.dev
]
