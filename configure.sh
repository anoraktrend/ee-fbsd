#!/bin/sh

# Detect OS
os_type=$(uname)
case "$os_type" in 
    FreeBSD|NetBSD|OpenBSD)
        echo "Configuring for BSD system..."
        have_bsd=1
        ;;
    Linux)
        echo "Configuring for Linux system..."
        have_linux=1
        ;;
    *)
        echo "Unknown system type: $os_type"
        exit 1
        ;;
esac

# Create config.h
cat > config.h << EOF
#ifndef CONFIG_H
#define CONFIG_H

/* System configuration */
#ifdef __FreeBSD__
#define HAVE_BSD 1
#endif

/* Terminal handling */
#ifdef HAVE_BSD
#define USE_SGTTY 1
#else
#define USE_TERMIO 1
#endif

/* Screen attributes */
#define HAVE_STANDOUT 1
#define HAVE_BIG5 1

#endif /* CONFIG_H */
EOF

chmod +x configure.sh

echo "Configuration complete"
