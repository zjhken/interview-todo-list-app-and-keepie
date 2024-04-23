This project is for the interviewers to implement the todo list APP.

# Task details
You are going to implement a todo-list app with the below requirements:
* (basic CRUD)Implement "add task", "get task", "list tasks", "delete tasks","modify task" endpoints.
* Each endpoint should be protect by a simple username and password.
* This app will not store any username and password, this app should get the credential information from a service that storing credentials, let's call this credential service "Keepie".(I've made this Keepie service ready for you)
* Because of network problem, the Keepie server will not always send the credential to the todo-list APP successfully. You need to keep calling Keepie server for credential until you APP receive it.
* Once your APP receives the credential, it should stop calling Keepie server to prevent unnecessary network traffic.

# How Keepie works
Here are the steps for the APP to retrieve the credentials from Keepie.
1. Todo-list APP makes an HTTP call to Keepie server, with the "receive_url" value in the HTTP body. Keepie server replies with 200.
Bash example
```bash
curl -X POST -d '{"receive_url": "http://localhost:9999/givemesecret"}' http://localhost:8000/sendSecretToMe
# Hello Some("127.0.0.1:60898") ! I've send the secrets to http://localhost:9999/givemesecret
```
2. Keepie server send the credential with a POST request to the "receive_url"
This step will not always work due the network stability. My Keepie server will simulate the network failure.
3. Todo-list APP receives the credential and store it in memory, and use this credential to validate all the todo-list CRUD requests.


# How to run this Keepie server in local
Simple download the binary in the Github release. I've compile the x64 GNU Linux binary for you.
Run the binary in your local in a Linux environment
`./interview-keepie`

# Can I use it without downloading the binary?
Yes. I've deployed this app to a server with public IP.
You can just specify the Keepie server to http://107.173.104.196:8000


# How to test 
The below script will try to call the Keepie server 10 times.
```bash
KEEPIE_SERVER=http://localhost:8000 # for local
KEEPIE_SERVER=http://107.173.104.196:8000 # for internet

for n in {1..10} ; do curl -X POST -d '{"receive_url": "http://localhost:9999/givemesecret"}' $KEEPIE_SERVER/sendSecretToMe; sleep 1; done
```

The output of the above script will be like below. After this call, Keepie server will try to send the credential to the recieve_url. But there will be only 20% of them will be successful.. This is to simulate the network problem.
```
Hello Some("127.0.0.1:60888") ! I've send the secrets to  http://localhost:9999/givemesecret
```

And once Keepie successfully send the credential to the todo-list APP, the APP side will receive the HTTP request like below
```
POST /givemesecret HTTP/1.1
Host: localhost:9999
Accept: */*
Accept-Encoding: deflate, gzip
content-type:application/json
user-agent:curl/8.6.0-DEV isahc/0.9.14
Content-Length: 45

{"username":"user_a","password":"password_a"}
```

# Contact
If you got any problem or want to ask anything, please send mail to zjhken@outlook.com