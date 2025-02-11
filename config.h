
#ifndef CONFIG_H
#define CONFIG_H

/* System configuration */
#if defined(__FreeBSD__) || defined(__NetBSD__) || defined(__OpenBSD__)
#define HAVE_BSD 1
#elif defined(__linux__)
#define HAVE_LINUX 1
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
