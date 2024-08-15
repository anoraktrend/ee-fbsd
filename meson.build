# Placed into the public domain via the Unlicense
project('ee', 'c', # change if/when rust is ready
  version : '1.5.1',    # source: trust me
  default_options : ['warning_level=3']) # autogen

cc = meson.get_compiler('c')
fs = import('fs')
host_sys = host_machine.system()

# Do we have termcap?
# FreeBSD
if fs.exists('/etc/termcap')
  add_project_arguments('-DTERMCAP=\\"/etc/termcap\\"', language : 'c')
# Various Linux distros.
elif fs.exists('/usr/share/lib/termcap')
  add_project_arguments('-DTERMCAP=\\"/usr/share/misc/termcap\\"', language : 'c')
elif fs.exists('/usr/share/misc/termcap')
  add_project_arguments('-DTERMCAP=\\"/usr/share/misc/termcap\\"', language : 'c')
endif

# Now terminfo
if fs.exists('/usr/lib/terminfo') or fs.exists('/usr/share/lib/terminfo') or fs.exists('/usr/share/terminfo')
  add_project_arguments('-DCAP', language : 'c')
endif

# SysV termino header
if cc.check_header('termio.h')
  add_project_arguments('-DSYS5', language : 'c')
endif

# Do we have select function?
# Probably need more robust checking if we
# want this to work on things like AIX.
if cc.has_function('select')
  add_project_arguments('-DBSD_SELECT', language : 'c')
endif

# We *could* check if we have these headers,
# but i think we'll be OK just this once if we don't.
add_project_arguments('-DHAS_UNISTD', language : 'c')
add_project_arguments('-DHAS_STDLIB', language : 'c')
add_project_arguments('-DHAS_STDARG', language : 'c')
add_project_arguments('-DHAS_CTYPE', language : 'c')
add_project_arguments('-DHAS_SYS_IOCT', language : 'c')
add_project_arguments('-DHAS_SYS_WAIT', language : 'c')

# Localization
if not cc.check_header('locale.h') and not cc.check_header('nl_types.h')
  add_project_arguments('-DNO_CATGETS', language : 'c')
endif

# TODO: Handle SunOS 5lib


# MacOS check
if host_sys == 'darwin'
  add_project_arguments('-DNO_CATGETS', language : 'c')
endif

# Musl quirk
# There is no easy way to detect if we're using musl.
# We could ask ldd, but that seems fragile and not very
# cross-platform. Instead, pass -Dusing-musl=true to meson
# when building on a musl platform.
if get_option('using-musl') == true
  add_project_arguments('-DNO_CATGETS', language : 'c')
endif

# Still, we can try to guess.
musl_test = '''
#include <stdlib.h>
#if (defined __GLIBC__) || (defined __UCLIBC__)
    #error we are using musl... hopefully
#endif
'''

if host_sys == 'linux' and cc.compiles(musl_test, name :'testing for musl libc')
  add_project_arguments('-DNO_CATGETS', language : 'c')
endif

# Trying to build new_curses from tree seems to be a
# rather hard sell. Let's just use the system provided ncurses.
curses_deps = dependency('ncursesw')

executable('ee',
           'ee.c',
           dependencies : [curses_deps],
           install : false)
