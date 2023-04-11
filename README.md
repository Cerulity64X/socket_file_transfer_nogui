# CLI File Transfer
A simple utility that allows you to transfer files over a TCP connection.

# Usage
Run with (\<exec\> -a {addr} -f {file} [-s])
The arguments are:\
-a {addr} (Specifies the address to connect/bind to.)\
-f {file name} (Specifies the file to save to/send.)\
-s (Flag, specifies send/receive. When provided, -f will read the file and -a will bind using that address. When not provided, -f will write to the file name provided, and -a will connect. This means that if no program is bound to -a, the program will halt after a few seconds. You must start the sender program before the host program to prevent this.)

# Networking issues
You may run into issues such as port forwarding/local & global addressing. These issues are much less likely to happen on local connections (e.g. two devices connected to the same home WiFi). For remote connections, if the host is not a public IP address, the receiver will not be able to connect. If the host does not have port forwarding, even if you have a public IP address, the program will likely crash.
