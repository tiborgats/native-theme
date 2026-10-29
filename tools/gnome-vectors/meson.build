project('gnome-vectors', 'c',
  default_options: ['c_std=gnu11', 'warning_level=1', 'buildtype=release'])

cc = meson.get_compiler('c')

executable('gnome-vectors', 'gnome-vectors.c',
  dependencies: [dependency('libadwaita-1', version: '>= 1.10'), cc.find_library('m')])
