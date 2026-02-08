#ifndef _UNISTD_H
#define _UNISTD_H

#include <stddef.h>

typedef int pid_t;
typedef unsigned int uid_t;
typedef unsigned int gid_t;
typedef long ssize_t;
typedef long off_t;

#define STDIN_FILENO  0
#define STDOUT_FILENO 1
#define STDERR_FILENO 2

/* File access */
#define R_OK 4
#define W_OK 2
#define X_OK 1
#define F_OK 0

ssize_t read(int fd, void *buf, size_t count);
ssize_t write(int fd, const void *buf, size_t count);
int close(int fd);
off_t lseek(int fd, off_t offset, int whence);
int access(const char *pathname, int mode);
int unlink(const char *pathname);

/* Process */
pid_t fork(void);
pid_t getpid(void);
pid_t getppid(void);
uid_t getuid(void);
gid_t getgid(void);
int execv(const char *pathname, char *const argv[]);
int execve(const char *pathname, char *const argv[], char *const envp[]);
int execvp(const char *file, char *const argv[]);
void _exit(int status);

/* Directory */
int chdir(const char *path);
char *getcwd(char *buf, size_t size);

/* Sleep */
unsigned int sleep(unsigned int seconds);
int usleep(unsigned int usec);

/* Other */
int isatty(int fd);
int dup(int oldfd);
int dup2(int oldfd, int newfd);
int pipe(int pipefd[2]);

#endif /* _UNISTD_H */
