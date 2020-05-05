
## tokio-tcp-server example

An example with tokio library 0.1.22.
The tcp client sends 12 bytes of data cyclically, the tcp server loses data.

### Listing
```
TCP Server
listening...TcpListener { addr: V4(0.0.0.0:502), fd: 3 }
Conected: 127.0.0.1:51086
2020-05-05 22:20:48.239102348 +02:00 Error: Len is Incorrect
Conected: 127.0.0.1:51122
2020-05-05 22:21:30.520593395 +02:00 Error: Len is Incorrect
Conected: 127.0.0.1:51140
2020-05-05 22:22:12.757771345 +02:00 Error: Len is Incorrect
Conected: 127.0.0.1:51144
2020-05-05 22:22:55.021565442 +02:00 Error: Len is Incorrect
Conected: 127.0.0.1:51150
2020-05-05 22:23:37.282017740 +02:00 Error: Len is Incorrect
Conected: 127.0.0.1:51156
```

