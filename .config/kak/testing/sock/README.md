Glua - glue between the kakoune editor and the lua programming language.

This will spawn Glua server in <path> if specified, otherwise will spawn it in /tmp
server is basically directory with unix socket and file in which stored glua pid.
If there is no problems path of spawned server will be showed as output

- glua spawn <path>

or

- glua kakspawn <path>
Output of this command will be kakone module with commands:
    glua-kill - kill current lua server
    glua-eval - execute chunck with provided arguments
    glua-eval-sync - this will block editor while lua chunck executing
and option in which stored server path, which will be used to for glua-module commands
    %opt{GLUA_socket_root}

This will remove server directory in <path> if specified, otherwise will try to find
server root directory in /tmp and if there is one or more will delete them all

- glua kill <path>


