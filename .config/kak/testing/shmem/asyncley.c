#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <stdbool.h>
#include "luajit.h"
#include "lualib.h"
#include "lauxlib.h"
#include "lua.h"

void panic(char* msg) {
    fprintf(stderr, "%s\n", msg);
    exit(1);
}

// figure out how to pass command array into function
void pipe_msg(char* argv[], char* msg) {
    int fd[2];
    if (pipe(fd) == -1) 
        panic("failed to create pipe"); 

    pid_t pid = fork();
    if (pid < 0) 
        panic("failed to fork");

    // chiled process
    if (pid == 0) { 
        // close unused "write" part of pipe
    	close(fd[1]);
    	// redirect stdin to read from "read" part of pipe
    	if (dup2(fd[0], STDIN_FILENO) == -1) 
        	panic("failed to redirect msg to stdin"); 
    	close(fd[0]);
		execvp(argv[0], argv);
    } else {
    // parent process
		close(fd[0]);
		if (write(fd[1], msg, strlen(msg)) == -1) 
    		panic("failed to write to the pipe");
		close(fd[1]);
    }
}

// TODO: TRY TO PIPE LUA OUT TO KAKOUNE
int run() {
    char MYFIFO[] = "myfifo"; 
    mkfifo(MYFIFO, 0666);
    int file_desc = open(MYFIFO, O_RDONLY);

    if (file_desc == -1) {
        perror("fifo open");
        exit(1);
    }

    lua_State* L = luaL_newstate();

    if (!luaJIT_setmode(L, 0, LUAJIT_MODE_ON)) {
        perror("luajit init");
        exit(1);
    }

	luaL_openlibs(L);

    size_t buf_size = 1024;
    int read_bytes;
    char buffer[buf_size];
	while (1) {
        read_bytes = read(file_desc, buffer, buf_size);
        buffer[read_bytes] = '\0';

        if ((strcmp(buffer, "STOP") == 0)) break;

    	if (luaL_dostring(L, buffer) != LUA_OK) {
    		fprintf(stdout, "Error: %s\n", lua_tostring(L, -1));
    		lua_pop(L, 1);
    		continue;
    	}

    	lua_pop(L, 1);

    	buffer[0] = '\0';
    	usleep(200 * 1000);
    }

	close(file_desc);
	lua_close(L);
	unlink(MYFIFO);

    return 0;
}

int main() {
    char* ses = "kak";
    char* msg = "eval -client client0 %{ echo -debug hello }"; 
    // TODO: DOESNT WORK FIX
    char command[] = { "/usr/kak", "-p" }; 
	pipe_msg(command, msg);
    return 0;
}
